use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use sqlx::{
    sqlite::SqlitePool as Pool, FromRow};
use dotenv::dotenv;

use tower_http::trace::TraceLayer;
use tracing_subscriber;

use utoipa::{ToSchema, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};


#[derive(FromRow, Serialize, ToSchema)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
struct Error {
    error: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(index, customers, customer),
    components(schemas(Customer, Error)),
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    dotenv().ok();
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::connect(&db_connection_str).await?;

    let api_router = Router::new()
        .route("/customers", get(customers))
        .route("/customer/:id", get(customer))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let app = Router::new()
        .route("/", get(index))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api", api_router);
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

#[utoipa::path(get, path="/", responses((status = 200, description = "Welcome to the customer database", body = String)))]
async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to the customer database</h1>")
}

#[utoipa::path(get, path="/api/customers", responses((status = 200, description = "List of customers", body = [Customer])))]
async fn customers(
    State(pool): State<Pool>,) -> Result<Json<Vec<Customer>>, Response> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customer")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DbError: {:?}", e);
            Json(Error {error: format!("{:?}", e)}).into_response()
        })?;
    Ok(Json(customers))
}

#[utoipa::path(
    get,
    path = "/api/customer/{id}",
    params(
        ("id" = i32, Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Get customer by ID", body = Customer),
        (status = 404, description = "Customer not found", body = Error)
    )
)]
async fn customer(
    Path(id): Path<i32>,
    State(pool): State<Pool>, 
) -> Result<Json<Customer>, Response> {
        let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customer WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DbError: {:?}", e);
            Json(Error {error: format!("{:?}", e)}).into_response()
        })?;
    Ok(Json(customer))
}

