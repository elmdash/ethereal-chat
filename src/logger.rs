use tracing::Level;

pub fn init() {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(Level::DEBUG)
        .init();
}
