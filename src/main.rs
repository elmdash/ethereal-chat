use axum::handler::get;
use axum::Router;
use ethereal_chat::{logger, SessionLayer};
use std::net::SocketAddr;
use tracing::info;

async fn list() {}

#[tokio::main]
async fn main() {
    logger::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(SessionLayer::build());

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
