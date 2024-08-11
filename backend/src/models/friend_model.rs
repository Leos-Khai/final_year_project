use serde::Serialize;
use sqlx::{FromRow, PgPool, Result};

#[derive(FromRow, Debug, Serialize)]
pub struct Friend {
    pub user_id1: i32,
    pub user_id2: i32,
    pub confirmed: bool,
}

impl Friend {
    // Add a new friend request
    pub async fn add_friend_request(pool: &PgPool, user_id1: i32, user_id2: i32) -> Result<Self> {
        let friend_request = sqlx::query_as!(
            Friend,
            r#"
      INSERT INTO friends (user_id1, user_id2, confirmed)
      VALUES ($1, $2, FALSE)
      RETURNING user_id1, user_id2, confirmed AS "confirmed!: bool"
      "#,
            user_id1,
            user_id2
        )
        .fetch_one(pool)
        .await?;

        Ok(friend_request)
    }

    // Confirm a friend request
    pub async fn confirm_friend_request(
        pool: &PgPool,
        user_id1: i32,
        user_id2: i32,
    ) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE friends
            SET confirmed = TRUE
            WHERE user_id1 = $1 AND user_id2 = $2
            "#,
            user_id1,
            user_id2
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    // Delete a friend (unfriend)
    pub async fn delete_friend(pool: &PgPool, user_id1: i32, user_id2: i32) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM friends
            WHERE (user_id1 = $1 AND user_id2 = $2) OR (user_id1 = $2 AND user_id2 = $1)
            "#,
            user_id1,
            user_id2
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    // Get a list of friends for a user
    pub async fn get_friends(pool: &PgPool, user_id: i32) -> Result<Vec<Friend>> {
        let friends = sqlx::query_as!(
            Friend,
            r#"
      SELECT user_id1, user_id2, confirmed AS "confirmed!: bool"
      FROM friends
      WHERE (user_id1 = $1 OR user_id2 = $1) AND confirmed = TRUE
      "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(friends)
    }

    // Check if two users are friends
    pub async fn are_friends(pool: &PgPool, user_id1: i32, user_id2: i32) -> Result<bool> {
        let friend = sqlx::query!(
            r#"
          SELECT confirmed
          FROM friends
          WHERE (user_id1 = $1 AND user_id2 = $2) OR (user_id1 = $2 AND user_id2 = $1)
          "#,
            user_id1,
            user_id2
        )
        .fetch_optional(pool)
        .await?;

        Ok(friend.map_or(false, |f| f.confirmed.unwrap_or(false)))
    }
}
