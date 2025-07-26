use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use crate::common::data_access_layer::{DatabaseCredentials};
pub async fn setup_container(credentials: DatabaseCredentials) -> ContainerAsync<GenericImage> {
    tracing::info!("Starting MasjidAppDatabase");
    let container = GenericImage::new("masjidappdatabase", "latest")
        .with_exposed_port(3306.tcp())
        .with_wait_for(WaitFor::seconds(30))
        .with_wait_for(WaitFor::message_on_stderr(
            "/usr/sbin/mysqld: ready for connections",
        ))
        .with_env_var("MYSQL_ROOT_PASSWORD", "password")
        .start()
        .await
        .unwrap();
    tracing::info!("MasjidAppDatabase started successfully");
    let username = credentials.username;
    let password = credentials.password;
    let port = container.get_host_port_ipv4(3306).await.unwrap();
    let connection_string =
        format!("mysql://{username}:{password}@127.0.0.1:{port}/masjidappdatabase");
    tracing::debug!(
        "Setting environment variable {} value to {}",
        &credentials.environment_variable,
        &connection_string
    );
    unsafe {
        std::env::set_var(credentials.environment_variable, connection_string);
    }
    container
}