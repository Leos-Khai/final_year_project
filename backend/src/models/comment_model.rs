use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Comment {
    pub comment_id: i32,
    pub comment_content: String,
    pub comment_date: Option<NaiveDateTime>,
    pub author_type: String,
    pub author_id: i32,
    pub author_name: String,
    pub post_id: i32,
}

impl Comment {
    // Create a new comment
    pub async fn create(pool: &PgPool, new_comment: Comment) -> Result<Self> {
        let created_comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (comment_content, comment_date, author_type, author_id, post_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING comment_id, comment_content, comment_date, author_type, author_id, post_id,
                      COALESCE(
                          CASE
                              WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                              WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                          END, ''
                      ) AS "author_name!"
            "#,
            new_comment.comment_content,
            Some(Utc::now().naive_utc()), // Using chrono's Utc::now() for comment_date
            new_comment.author_type,
            new_comment.author_id,
            new_comment.post_id
        )
        .fetch_one(pool)
        .await?;

        Ok(created_comment)
    }

    // Fetch a comment by ID
    pub async fn find_by_id(pool: &PgPool, comment_id: i32) -> Result<Self> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT comment_id, comment_content, comment_date, author_type, author_id, post_id,
                   COALESCE(
                       CASE
                           WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                           WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                       END, ''
                   ) AS "author_name!"
            FROM comments
            WHERE comment_id = $1
            "#,
            comment_id
        )
        .fetch_one(pool)
        .await?;

        Ok(comment)
    }

    // Fetch all comments for a specific post
    pub async fn find_by_post_id(pool: &PgPool, post_id: i32) -> Result<Vec<Self>> {
        let comments = sqlx::query_as!(
            Comment,
            r#"
            SELECT comment_id, comment_content, comment_date, author_type, author_id, post_id,
                   COALESCE(
                       CASE
                           WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                           WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                       END, ''
                   ) AS "author_name!"
            FROM comments
            WHERE post_id = $1
            ORDER BY comment_date ASC
            "#,
            post_id
        )
        .fetch_all(pool)
        .await?;

        Ok(comments)
    }

    // Update a comment
    pub async fn update(pool: &PgPool, comment_id: i32, updated_content: String) -> Result<Self> {
        let updated_comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET comment_content = $1, comment_date = $2
            WHERE comment_id = $3
            RETURNING comment_id, comment_content, comment_date, author_type, author_id, post_id,
                      COALESCE(
                          CASE
                              WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                              WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                          END, ''
                      ) AS "author_name!"
            "#,
            updated_content,
            Some(Utc::now().naive_utc()),
            comment_id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_comment)
    }

    // Delete a comment
    pub async fn delete(pool: &PgPool, comment_id: i32) -> Result<u64> {
        let deleted_rows = sqlx::query!(
            r#"
            DELETE FROM comments WHERE comment_id = $1
            "#,
            comment_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(deleted_rows)
    }
}
