pub mod mysql;
pub mod redis;

pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub environment_variable: String,
}
