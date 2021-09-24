use tracing_subscriber::fmt::format::FmtSpan;

pub fn init() {
    tracing_subscriber::fmt().without_time().init();
}
