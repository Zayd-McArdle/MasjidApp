use tracing_subscriber::EnvFilter;

pub fn setup() {
    let subscriber = tracing_subscriber::fmt();
    let deployment_environment =
        std::env::var("DEPLOYMENT_ENV").expect("DEPLOYMENT_ENV should be set");
    if deployment_environment == "TESTING" {
        subscriber
            .with_env_filter(EnvFilter::new("debug"))
            .pretty()
            .init();
    } else if deployment_environment == "PRODUCTION" {
        subscriber.json().init();
    } else {
        panic!("Unsupported deployment environment")
    }
}
