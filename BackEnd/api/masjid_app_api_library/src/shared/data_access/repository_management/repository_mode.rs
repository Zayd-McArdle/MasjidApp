use crate::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use crate::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum RepositoryMode {
    InMemory(InMemoryDbProvider),
    Normal(NormalDbProvider),
}
