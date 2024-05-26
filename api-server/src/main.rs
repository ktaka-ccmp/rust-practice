use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse, Json, Response},
    routing::get,
    Router,
    // Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use sqlx::{
    sqlite::SqlitePool as Pool, FromRow};
use dotenv::dotenv;

use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    dotenv().ok();
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::connect(&db_connection_str).await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/customers", get(customers))
        .route("/customer/:id", get(customer))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to the customer database</h1>")
}

#[derive(Serialize)]
struct Error {
    error: String,
}

async fn customers(
    State(pool): State<Pool>,) -> Result<Json<Vec<Customer>>, Response> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customer")
        .fetch_all(&pool)
        .await
        .map_err(|e| Json(Error {error: format!("{:?}", e)}).into_response())?;
    Ok(Json(customers))
}

async fn customer(
    Path(id): Path<i32>,
    State(pool): State<Pool>, 
) -> Result<Json<Customer>, Response> {
        let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customer WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| Json(Error {error: format!("{:?}", e)}).into_response())?;
    Ok(Json(customer))
}
