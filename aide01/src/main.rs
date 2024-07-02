use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
    scalar::Scalar,
};
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
struct User {
    name: String,
}

async fn hello_user(Json(user): Json<User>) -> impl IntoApiResponse {
    format!("hello {}", user.name)
}

async fn hello() -> impl IntoApiResponse {
    "hello world"
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[tokio::main]
async fn main() {
    let app = ApiRouter::new()
        // Change `route` to `api_route` for the route
        // we'd like to expose in the documentation.
        .route("/docs", Scalar::new("/api.json").axum_route())
        .api_route("/", get(hello))
        .api_route("/hello", post(hello_user))
        // We'll serve our generated document here.
        .route("/api.json", get(serve_api));

    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    axum::serve(
        listener,
        app.finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
