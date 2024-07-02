use aide::axum::{routing::get, ApiRouter};
use askama_axum::Template;
use axum::extract::Path;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.j2")]
struct IndexTemplate {
    title: String,
}

async fn index() -> Html<String> {
    let index_template = IndexTemplate {
        title: "Hello, World!".to_string(),
    };
    let template = index_template;
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "spa.j2")]
struct SpaTemplate {
    title: String,
    page: String,
}

async fn get_spa(Path(page): Path<String>) -> Html<String> {
    let template = SpaTemplate {
        title: page.clone(),
        page: page,
    };
    Html(template.render().unwrap())
}

pub fn create_router() -> ApiRouter {
    ApiRouter::new()
        .api_route("/index", get(index))
        .api_route("/:page", get(get_spa))
    }
