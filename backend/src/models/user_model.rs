use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
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
                email, 
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

    /// Fetch a user by email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id, 
                username, 
                email, 
                password_hash, 
                user_type
            FROM user_authentication
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Fetch a user by ID
    pub async fn find_by_id(pool: &PgPool, user_id: i32) -> Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
              SELECT 
                  user_id, 
                  username, 
                  email, 
                  password_hash, 
                  user_type
              FROM user_authentication
              WHERE user_id = $1
              "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Update a user's password
    pub async fn update_password(pool: &PgPool, user_id: i32, password_hash: &str) -> Result<()> {
        sqlx::query!(
            r#"
              UPDATE member
              SET password_hash = $1
              WHERE member_id = $2
              "#,
            password_hash,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
