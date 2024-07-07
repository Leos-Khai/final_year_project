use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub user_type: Option<String>,
}

impl User {
    /// Fetch a user by username
    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id, 
                username, 
                password_hash, 
                user_type
            FROM user_authentication
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
