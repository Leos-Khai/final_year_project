use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Member {
    pub member_id: i32,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub profile_pic: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewMember {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub profile_pic: Option<String>,
}

impl Member {
    /// Create a new member
    pub async fn create(pool: &PgPool, new_member: NewMember) -> SqlxResult<Self> {
        let created_member = sqlx::query_as!(
            Member,
            r#"
            INSERT INTO member (username, email, password_hash, full_name, phone_number, profile_pic)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING member_id, username, email, full_name, phone_number, profile_pic
            "#,
            new_member.username,
            new_member.email,
            new_member.password_hash, // Insert password_hash, but do not return it
            new_member.full_name,
            new_member.phone_number,
            new_member.profile_pic
        )
        .fetch_one(pool)
        .await?;

        Ok(created_member)
    }

    /// Get a member by ID without returning the password hash
    pub async fn get_by_id(pool: &PgPool, member_id: i32) -> SqlxResult<Self> {
        let member = sqlx::query_as!(
            Member,
            r#"
            SELECT member_id, username, email, full_name, phone_number, profile_pic
            FROM member
            WHERE member_id = $1
            "#,
            member_id
        )
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    /// Update a member by ID without updating the password hash
    pub async fn update(pool: &PgPool, member_id: i32, updated_member: Member) -> SqlxResult<Self> {
        let updated_member = sqlx::query_as!(
            Member,
            r#"
            UPDATE member
            SET username = $1, email = $2, full_name = $3, phone_number = $4, profile_pic = $5
            WHERE member_id = $6
            RETURNING member_id, username, email, full_name, phone_number, profile_pic
            "#,
            updated_member.username,
            updated_member.email,
            updated_member.full_name,
            updated_member.phone_number,
            updated_member.profile_pic,
            member_id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_member)
    }

    /// Verify the old password is correct
    pub async fn verify_password(
        pool: &PgPool,
        member_id: i32,
        password: &str,
    ) -> SqlxResult<bool> {
        let stored_hash = sqlx::query_scalar!(
            r#"
          SELECT password_hash FROM member WHERE member_id = $1
          "#,
            member_id
        )
        .fetch_one(pool)
        .await?;

        match verify(password, &stored_hash) {
            Ok(is_valid) => Ok(is_valid),
            Err(_) => Ok(false), // Treat verification errors as "password doesn't match"
        }
    }

    /// Update the password for a member
    pub async fn update_password(
        pool: &PgPool,
        member_id: i32,
        old_password: &str,
        new_password: &str,
    ) -> SqlxResult<u64> {
        // Verify the old password
        if !Self::verify_password(pool, member_id, old_password).await? {
            return Err(sqlx::Error::RowNotFound); // You can use a more descriptive error here
        }

        // Hash the new password
        let new_password_hash = match hash(new_password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e,
                )))
            }
        };

        // Update the password in the database
        let rows_affected = sqlx::query!(
            r#"
          UPDATE member
          SET password_hash = $1
          WHERE member_id = $2
          "#,
            new_password_hash,
            member_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    /// Delete a member by ID
    pub async fn delete(pool: &PgPool, member_id: i32) -> SqlxResult<u64> {
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
