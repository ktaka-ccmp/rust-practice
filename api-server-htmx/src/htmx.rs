use aide::axum::{routing::get, ApiRouter};
use aide::openapi::Response;
use askama_axum::Template;
// use askama_axum::Response;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};
use sqlx::SqlitePool;

use crate::models::{Customer, Params};

#[derive(Template)]
#[template(path = "content.list.j2")]
struct ContentListTemplate {
    title: String,
    skip_next: i32,
    limit: i32,
}

async fn content_list() -> Html<String> {
    let template = ContentListTemplate {
        title: "Incremental hx-get demo".to_string(),
        skip_next: 0,
        limit: 2,
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "content.list.tbody.j2")]
struct ContentListTbodyTemplate {
    skip_next: i32,
    limit: i32,
    customers: Vec<Customer>,
}

async fn content_list_tbody(
    Query(params): Query<Params>,
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<Html<String>, StatusCode> {
    if headers.get("HX-Request").is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let skip = params.skip.unwrap_or(0);
    let limit = params.limit.unwrap_or(1);

    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customer LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(skip)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let template = ContentListTbodyTemplate {
        skip_next: skip + limit,
        limit,
        customers,
    };

    Ok(Html(template.render().unwrap()))
}

pub fn create_router(pool: SqlitePool) -> ApiRouter {
    ApiRouter::new()
        .api_route("/content.list", get(content_list))
        .api_route("/content.list.tbody", get(content_list_tbody))
        .with_state(pool)
}
