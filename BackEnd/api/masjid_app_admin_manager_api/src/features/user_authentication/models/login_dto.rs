#[derive(Debug, sqlx::FromRow, Clone, PartialEq)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
    pub role: String,
}
