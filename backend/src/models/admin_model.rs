use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Admin {
    pub admin_id: i32,
    pub username: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub profile_pic: Option<String>,
}

impl Admin {
    /// Fetch an admin by username
    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Self> {
        let admin = sqlx::query_as!(
            Admin,
            r#"
            SELECT admin_id, username, password_hash, full_name, phone_number, profile_pic
            FROM admin
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(admin)
    }
}
