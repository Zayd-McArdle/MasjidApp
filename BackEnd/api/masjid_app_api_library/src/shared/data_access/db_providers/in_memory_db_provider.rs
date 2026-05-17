use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum InMemoryDbProvider {
    Redis,
}
