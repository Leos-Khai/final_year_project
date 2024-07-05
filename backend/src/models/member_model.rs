use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Member {
    pub member_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub profile_pic: Option<String>,
}

impl Member {
    /// Create a new member
    pub async fn create(pool: &PgPool, new_member: Member) -> Result<Self> {
        let created_member = sqlx::query_as!(
            Member,
            r#"
            INSERT INTO member (username, email, password_hash, full_name, phone_number, profile_pic)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING member_id, username, email, password_hash, full_name, phone_number, profile_pic
            "#,
            new_member.username,
            new_member.email,
            new_member.password_hash,
            new_member.full_name,
            new_member.phone_number,
            new_member.profile_pic
        )
        .fetch_one(pool)
        .await?;

        Ok(created_member)
    }

    /// Get a member by ID
    pub async fn get_by_id(pool: &PgPool, member_id: i32) -> Result<Self> {
        let member = sqlx::query_as!(
            Member,
            r#"
            SELECT member_id, username, email, password_hash, full_name, phone_number, profile_pic
            FROM member
            WHERE member_id = $1
            "#,
            member_id
        )
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    /// Update a member by ID
    pub async fn update(pool: &PgPool, member_id: i32, updated_member: Member) -> Result<Self> {
        let updated_member = sqlx::query_as!(
            Member,
            r#"
            UPDATE member
            SET username = $1, email = $2, password_hash = $3, full_name = $4, phone_number = $5, profile_pic = $6
            WHERE member_id = $7
            RETURNING member_id, username, email, password_hash, full_name, phone_number, profile_pic
            "#,
            updated_member.username,
            updated_member.email,
            updated_member.password_hash,
            updated_member.full_name,
            updated_member.phone_number,
            updated_member.profile_pic,
            member_id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_member)
    }

    /// Delete a member by ID
    pub async fn delete(pool: &PgPool, member_id: i32) -> Result<u64> {
        let deleted_rows = sqlx::query!(
            r#"
            DELETE FROM member
            WHERE member_id = $1
            "#,
            member_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(deleted_rows)
    }
}
