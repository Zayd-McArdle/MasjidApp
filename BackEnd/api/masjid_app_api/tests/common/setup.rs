use masjid_app_api::shared::repository_manager::RepositoryMode;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub environment_variable: String,
}
impl DatabaseCredentials {
    pub async fn new(username: String, password: String, environment_variable: String) -> Self {
        Self {
            username,
            password,
            environment_variable,
        }
    }
}
fn initialise_connection_string_environment_variables(repository_mode: RepositoryMode) {
    match repository_mode {
        RepositoryMode::InMemory => {
            todo!()
        }
        RepositoryMode::Normal => {}
    }
    unsafe {}
}
pub async fn setup_main_database(credentials: DatabaseCredentials) -> ContainerAsync<GenericImage> {
    println!("Starting MasjidAppDatabase");
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
    println!("MasjidAppDatabase started successfully");
    let username = credentials.username;
    let password = credentials.password;
    let port = container.get_host_port_ipv4(3306).await.unwrap();
    let connection_string =
        format!("mysql://{username}:{password}@127.0.0.1:{port}/masjidappdatabase");
    println!(
        "Setting environment variable \"{}\" to \"{}\"",
        &credentials.environment_variable, &connection_string
    );
    unsafe {
        std::env::set_var(credentials.environment_variable, connection_string);
    }
    container
}
