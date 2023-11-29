use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

async fn home() -> HomeTemplate {
    HomeTemplate {}
}

#[derive(Template)]
#[template(path = "content.html")]
struct ContentTemplate<'a> {
    name: &'a str,
}

async fn content() -> ContentTemplate<'static> {
    ContentTemplate { name: "World" }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(home))
        .route("/content", get(content));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
