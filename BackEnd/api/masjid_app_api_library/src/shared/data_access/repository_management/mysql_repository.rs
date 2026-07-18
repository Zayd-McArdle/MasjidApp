use crate::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use crate::shared::data_access::repository_management::repository_mode::RepositoryMode;
use crate::shared::data_access::repository_management::repository_type::RepositoryType;
use crate::shared::data_access::repository_management::utils::get_connection_string;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;

pub struct MySqlRepository {
    pub db_connection: Arc<MySqlPool>,
}

impl MySqlRepository {
    pub async fn new(repository_type: RepositoryType) -> Self {
        let connection_string = std::env::var(get_connection_string(
            repository_type,
            RepositoryMode::Normal(NormalDbProvider::MySql),
        ))
        .unwrap();
        let db_connection_result = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&connection_string)
            .await;
        match db_connection_result {
            Ok(db_connection) => {
                tracing::info!("database connection successfully established");
                Self {
                    db_connection: Arc::new(db_connection),
                }
            }
            Err(err) => {
                panic!("Failed to connect to database: {err}");
            }
        }
    }
}
