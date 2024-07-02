use aide::axum::{routing::get, ApiRouter};
use axum::{
    extract::{Path, State},
    response::{ IntoResponse, Response},
    Json,
};
use sqlx::SqlitePool;

use crate::models::{Customer, CustomerId, Error};

pub async fn customers(State(pool): State<SqlitePool>) -> Result<Json<Vec<Customer>>, Response> {
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

pub async fn customer(
    Path(cid): Path<CustomerId>,
    State(pool): State<SqlitePool>,
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

pub fn create_router(pool: SqlitePool) -> ApiRouter {
    ApiRouter::new()
        .api_route("/customers", get(customers))
        .api_route("/customer/:id", get(customer))
        .with_state(pool)
}
