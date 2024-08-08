use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct PasswordResetToken {
    pub token_id: i32,
    pub user_id: i32,
    pub reset_token: String,
    pub reset_token_expires: NaiveDateTime,
}

impl PasswordResetToken {
    /// Create a new reset token
    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        token: &str,
        expires: NaiveDateTime,
    ) -> Result<Self> {
        let token = sqlx::query_as!(
            PasswordResetToken,
            r#"
            INSERT INTO password_reset_tokens (user_id, reset_token, reset_token_expires)
            VALUES ($1, $2, $3)
            RETURNING token_id, user_id, reset_token, reset_token_expires
            "#,
            user_id,
            token,
            expires
        )
        .fetch_one(pool)
        .await?;

        Ok(token)
    }

    /// Fetch a reset token by token value
    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Self> {
        println!("Token {}", token);

        let r_token = sqlx::query_as!(
            PasswordResetToken,
            r#"
            SELECT token_id, user_id, reset_token, reset_token_expires
            FROM password_reset_tokens
            WHERE reset_token = $1
            "#,
            token
        )
        .fetch_one(pool)
        .await?;
        println!("R_Token: {:?}", r_token);
        Ok(r_token)
    }

    /// Delete a reset token by user ID
    pub async fn delete_by_user_id(pool: &PgPool, user_id: i32) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}
