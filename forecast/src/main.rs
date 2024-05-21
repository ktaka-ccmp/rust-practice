use axum::{routing::get, Router, async_trait, extract::{Query, State, FromRequestParts}, http::request::Parts};
use axum::response::{IntoResponse, Html};

use reqwest::StatusCode;
use std::net::SocketAddr;
use std::str::from_utf8;

use serde::Deserialize;
use askama_axum::Template;
use sqlx::{Error as SqlxError, PgPool};

use base64::{Engine as _, engine::general_purpose as BASE64};
use thiserror::Error;

#[derive(Deserialize)]
pub struct GeoResponse {
    pub results: Vec<LatLong>,
}
#[derive(sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub city: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
	pub latitude: f64,
	pub longitude: f64,
	pub timezone: String,
	pub hourly: Hourly,
}

#[derive(Deserialize, Debug)]
pub struct Hourly {
	pub time: Vec<String>,
	pub temperature_2m: Vec<f64>,
}

#[derive(Template, Deserialize, Debug)]
#[template(path = "weather.html")]
pub struct WeatherDisplay {
	pub city: String,
	pub forecasts: Vec<Forecast>,
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
	pub date: String,
	pub temperature: String,
}

impl WeatherDisplay {
    fn new(city: String, weather: WeatherResponse) -> Self {
		WeatherDisplay {
            city,
            forecasts: weather.hourly.time.iter().zip(weather.hourly.temperature_2m.iter()).map(|(time, temp)|
            Forecast {
                date: time.to_string(),
                temperature: temp.to_string(),
            }).collect(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "stats.html")]
struct StatsTemplate {
	pub cities: Vec<City>,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct City {
	pub name: String,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),
    #[error("City not found")]
    NotFound,
}

async fn index() -> IndexTemplate {
	IndexTemplate
}

async fn fetch_lat_long(city: &str) -> Result<LatLong, Box<dyn std::error::Error>> {
    let endpoint = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );
    let response = reqwest::get(&endpoint).await?.json::<GeoResponse>().await?;
    // response.results.get(0).cloned().ok_or("No results found".into())
    match response.results.get(0) {
        Some(lat_long) => Ok(lat_long.clone()),
        None => Err("No results found".into()),
    }
}

async fn get_lat_long(pool: &PgPool, name: &str) -> Result<LatLong, DbError> {
	let lat_long = sqlx::query_as::<_, LatLong>(
    	"SELECT lat::FLOAT8 AS latitude, long::FLOAT8 AS longitude FROM cities WHERE name = $1",
	)
	.bind(name)
	.fetch_optional(pool)
	.await?;

	if let Some(lat_long) = lat_long {
    	return Ok(lat_long);
	}

	// let lat_long = fetch_lat_long(name).await.map_err(|_| DbError::NotFound)?;
	let lat_long = match fetch_lat_long(name).await {
		Ok(lat_long) => lat_long,
		Err(e) => {
			println!("Error:City not found: {:?}", e);
			return Err(DbError::NotFound);
		},
	};

	println!("Inserting into database");
	sqlx::query("INSERT INTO cities (name, lat, long) VALUES ($1, $2, $3)")
    	.bind(name)
    	.bind(lat_long.latitude)
    	.bind(lat_long.longitude)
    	.execute(pool)
    	.await?;

	Ok(lat_long)
}

async fn fetch_weather(lat_long: LatLong) -> Result<WeatherResponse, reqwest::Error> {
	let endpoint = format!(
    	"https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
    	lat_long.latitude, lat_long.longitude
	);
	let response = reqwest::get(&endpoint).await?.json::<WeatherResponse>().await?;
	Ok(response)
}

async fn weather(
	Query(params): Query<WeatherQuery>,
	State(pool): State<PgPool>,
	) -> Result<impl IntoResponse, Html<String>> {
		let lat_long = match get_lat_long(&pool, &params.city).await {
		Ok(lat_long) => lat_long,
		Err(DbError::NotFound) => return Err(Html("City not found".to_string())),
		Err(_) => return Err(Html("Internal server error".to_string())),
	};
	let weather = fetch_weather(lat_long).await.map_err(|_| Html("Internal server error".to_string()))?;
	let weather_display = WeatherDisplay::new(params.city, weather);
	Ok(weather_display.into_response())
}

struct User;

#[async_trait]
impl<S> FromRequestParts<S> for User
where
	S: Send + Sync,
{
	type Rejection = axum::http::Response<axum::body::Body>;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
    	let auth_header = parts
        	.headers
        	.get("Authorization")
        	.and_then(|header| header.to_str().ok());

    	if let Some(auth_header) = auth_header {
        	if auth_header.starts_with("Basic ") {
            	let credentials = auth_header.trim_start_matches("Basic ");
				let decoded = BASE64::STANDARD.decode(credentials).unwrap();
            	let credential_str = from_utf8(&decoded).unwrap_or("");

            	// Our username and password are hardcoded here.
            	// In a real app, you'd want to read them from the environment.
            	if credential_str == "forecast:forecast" {
                	return Ok(User);
            	}
        	}
    	}

    	let reject_response = axum::http::Response::builder()
        	.status(StatusCode::UNAUTHORIZED)
        	.header(
            	"WWW-Authenticate",
            	"Basic realm=\"Please enter your credentials\"",
        	)
        	.body(axum::body::Body::from("Unauthorized"))
        	.unwrap();

    	Err(reject_response)
	}
}

async fn get_last_cities(pool: &PgPool) -> Result<Vec<City>, DbError> {
    let cities = sqlx::query_as::<_, City>("SELECT name FROM cities ORDER BY id DESC LIMIT 10")
        .fetch_all(pool)
        .await?;
    Ok(cities)
}

async fn stats(_user: User, State(pool): State<PgPool>) -> Result<StatsTemplate, StatusCode> {
	let cities = get_last_cities(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	Ok(StatsTemplate { cities })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	// let pool = sqlx::PgPool::connect(&db_connection_str).await.expect("Failed to connect to Postgres");
	let pool = sqlx::PgPool::connect(&db_connection_str).await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(weather))
        .route("/stats", get(stats))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
	Ok(())
}
