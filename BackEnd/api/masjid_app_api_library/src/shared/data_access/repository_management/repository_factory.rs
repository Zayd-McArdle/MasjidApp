#[macro_export]
macro_rules! new_repository {
    ($repository_mode:expr, $repository_type:expr) => {
        match $repository_mode {
            RepositoryMode::InMemory(in_memory_provider) => match in_memory_provider {
                InMemoryDbProvider::Redis => {
                    Arc::new(InMemoryRepository::new($repository_type).await)
                }
            },
            RepositoryMode::Normal(normal_provider) => match normal_provider {
                NormalDbProvider::MySql => Arc::new(MySqlRepository::new($repository_type).await),
            },
        }
    };
}
