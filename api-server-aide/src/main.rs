use aide::{
    axum::{
        routing::get,
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
    scalar::Scalar,
};

use axum::{
    Extension, Json,
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
};

use schemars::JsonSchema;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::SqlitePool as Pool, FromRow};
use std::net::SocketAddr;

use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[derive(FromRow, Serialize, JsonSchema)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// The additional CustomerId struct to make the id parameter required and documented.
#[derive(Deserialize, JsonSchema)]
struct CustomerId {
    /// The ID of the Customer.
    id: i32,
}
#[derive(Serialize)]
struct Error {
    error: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok();
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::connect(&db_connection_str).await?;

    let app = ApiRouter::new()
        .route("/docs", Scalar::new("/api.json").axum_route())
        .route("/api.json", get(serve_api))
        .api_route("/", get(index))
        .api_route("/customers", get(customers))
        .api_route("/customer/:id", get(customer))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(
        listener,
        app.finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await?;
    Ok(())
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to the customer database</h1>")
}

async fn customers(State(pool): State<Pool>) -> Result<Json<Vec<Customer>>, Response> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customer")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DbError: {:?}", e);
            Json(Error {
                error: format!("{:?}", e),
            })
            .into_response()
        })?;
    Ok(Json(customers))
}

async fn customer(
    Path(cid): Path<CustomerId>,
    State(pool): State<Pool>,
) -> Result<Json<Customer>, Response> {
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customer WHERE id = ?")
        .bind(cid.id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DbError: {:?}", e);
            Json(Error {
                error: format!("{:?}", e),
            })
            .into_response()
        })?;
    Ok(Json(customer))
}
