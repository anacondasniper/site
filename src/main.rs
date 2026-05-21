mod handlers;
mod templates;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(handlers::index))
        .nest_service("/static", ServeDir::new("static"));

    // .route("/thread{id}", axum::routing::get(handlers::thread));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
