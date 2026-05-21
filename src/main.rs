mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(handlers::index));
    // .route("/thread{id}", axum::routing::get(handlers::thread));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
