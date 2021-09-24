use ethereal_chat::logger;
use tracing::{info, debug, instrument};

#[instrument]
async fn hello() {
    println!("hello");
    info!("hello handled");
}

#[tokio::main]
async fn main() {
    logger::init();

    info!(test = 1, "trace info");
    let x = hello();
    debug!("trace debug");
    x.await;
}
