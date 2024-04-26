use axum::{routing::get, Router, extract::Query};
use reqwest::StatusCode;
use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GeoResponse {
    pub results: Vec<LatLong>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
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


async fn fetch_weather(lat_long: LatLong) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
	let endpoint = format!(
    	"https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
    	lat_long.latitude, lat_long.longitude
	);
	let response = reqwest::get(&endpoint).await?.json::<WeatherResponse>().await?;
	Ok(response)
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub city: String,
}

async fn weather(Query(params): Query<WeatherQuery>) -> Result<String, StatusCode> {
    // match fetch_lat_long(&params.city).await {
    //     Ok(lat_long) => Ok(format!("{}: {}, {}", params.city, lat_long.latitude, lat_long.longitude)),
    //     Err(_) => Err(StatusCode::NOT_FOUND),
    // }
    let lat_long = fetch_lat_long(&params.city).await.map_err(|_| StatusCode::NOT_FOUND)?;
    let weather = fetch_weather(lat_long).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let display = WeatherDisplay {
        city: params.city,
        forecasts: weather.hourly.time.iter().zip(weather.hourly.temperature_2m.iter()).map(|(time, temp)|
            Forecast {
                date: time.to_string(),
                temperature: temp.to_string(),
            }).collect(),
    };
    Ok(format!("{:?}", display))
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

#[derive(Deserialize, Debug)]
pub struct WeatherDisplay {
	pub city: String,
	pub forecasts: Vec<Forecast>,
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
	pub date: String,
	pub temperature: String,
}

async fn index() -> &'static str {
    "Index\n"
}

async fn weathers() -> &'static str {
    "Weathers"
}

async fn stats() -> &'static str {
    "Stats"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(weather))
        .route("/weathers", get(weathers))
        .route("/stats", get(stats));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
