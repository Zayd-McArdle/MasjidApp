#[derive(sqlx::FromRow, Clone)]
pub struct UserAccountDTO {
    pub full_name: String,
    pub email: String,
    pub role: String,
    pub username: String,
    pub password: String,
}
