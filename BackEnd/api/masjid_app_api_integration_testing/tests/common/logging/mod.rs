use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};

static INIT_LOGGER: Once = Once::new();

pub fn setup_logging() {
    INIT_LOGGER.call_once(|| {
        let subscriber = fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .pretty()
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    });
}