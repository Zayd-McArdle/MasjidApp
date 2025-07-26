pub mod redis;
pub mod mysql;

pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub environment_variable: String,
}