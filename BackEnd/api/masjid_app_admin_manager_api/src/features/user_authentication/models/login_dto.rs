#[derive(Debug, sqlx::FromRow, Clone)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
    pub role: String,
}
