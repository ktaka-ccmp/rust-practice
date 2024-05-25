use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use sqlx::{
    // error::Error as SqlxError,
    sqlite::SqlitePool as Pool, FromRow};
use dotenv::dotenv;

use std::convert::Infallible;
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
    axum::serve(listener, app).await?;
    Ok(())
}



async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to the customer database</h1>")
}

async fn customers(
    State(pool): State<Pool>,) -> Result<impl IntoResponse, Infallible> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customer")
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(Html(
        customers
            .iter()
            .map(|customer| format!("<h1>{}</h1><p>{}</p>", customer.name, customer.email))
            .collect::<Vec<_>>()
            .join(""),
    ))
}

async fn customer(
    Path(id): Path<i32>,
    State(pool): State<Pool>, 
) -> Result<impl IntoResponse, Infallible> {
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customer WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    Ok(Html(format!("<h1>{}</h1><p>{}</p>", customer.name, customer.email)))
}
