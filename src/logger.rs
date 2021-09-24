pub fn init() {
    tracing_subscriber::fmt().without_time().init();
}
