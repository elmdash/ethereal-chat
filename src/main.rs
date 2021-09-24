use ethereal_chat::logger;
use tracing::info;
use axum::Router;
use axum::handler::get;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    logger::init();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
