use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum NormalDbProvider {
    MySql,
}
