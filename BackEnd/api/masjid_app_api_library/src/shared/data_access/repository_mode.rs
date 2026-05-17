use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum RepositoryMode {
    InMemory,
    Normal,
}
