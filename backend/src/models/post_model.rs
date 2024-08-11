use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Post {
    pub post_id: i32,
    pub post_title: String,
    pub post_content: String,
    pub post_date: Option<NaiveDateTime>,
    pub like_count: Option<i32>,
    pub view_count: Option<i32>,
    pub author_type: String,
    pub author_id: i32,
    pub author_name: Option<String>, // Use Option<String> here
}

impl Post {
    /// Create a new post
    pub async fn create(pool: &PgPool, new_post: Post) -> Result<Self> {
        let created_post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (post_title, post_content, post_date, like_count, view_count, author_type, author_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id,
                      COALESCE(
                          CASE
                              WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                              WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                          END, NULL
                      ) AS author_name
            "#,
            new_post.post_title,
            new_post.post_content,
            Some(Utc::now().naive_utc()), // Using chrono's Utc::now() for post_date
            new_post.like_count.unwrap_or(0),
            new_post.view_count.unwrap_or(0),
            new_post.author_type,
            new_post.author_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Post {
            author_name: created_post.author_name.or(Some("".to_string())),
            ..created_post
        })
    }

    /// Fetch a post by ID
    pub async fn find_by_id(pool: &PgPool, post_id: i32) -> Result<Self> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id,
                   COALESCE(
                       CASE
                           WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                           WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                       END, NULL
                   ) AS author_name
            FROM posts
            WHERE post_id = $1
            "#,
            post_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Post {
            author_name: post.author_name.or(Some("".to_string())),
            ..post
        })
    }

    /// Update a post
    pub async fn update(pool: &PgPool, post_id: i32, updated_post: Post) -> Result<Self> {
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET post_title = $1, post_content = $2, post_date = $3, like_count = $4, view_count = $5, author_type = $6, author_id = $7
            WHERE post_id = $8
            RETURNING post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id,
                      COALESCE(
                          CASE
                              WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                              WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                          END, NULL
                      ) AS author_name
            "#,
            updated_post.post_title,
            updated_post.post_content,
            updated_post.post_date,
            updated_post.like_count,
            updated_post.view_count,
            updated_post.author_type,
            updated_post.author_id,
            post_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Post {
            author_name: post.author_name.or(Some("".to_string())),
            ..post
        })
    }

    /// Delete a post
    pub async fn delete(pool: &PgPool, post_id: i32) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM posts WHERE post_id = $1
            "#,
            post_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    // Like post
    pub async fn like_post(pool: &PgPool, user_id: i32, post_id: i32) -> Result<Self> {
        // Insert a new like record
        sqlx::query!(
            r#"
            INSERT INTO post_likes (user_id, post_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            user_id,
            post_id
        )
        .execute(pool)
        .await?;

        // Update the like count in the post table
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET like_count = like_count + 1
            WHERE post_id = $1
            RETURNING post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id,
                      COALESCE(
                          CASE
                              WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                              WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                          END, NULL
                      ) AS author_name
            "#,
            post_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Post {
            author_name: post.author_name.or(Some("".to_string())),
            ..post
        })
    }

    pub async fn increment_view_count(pool: &PgPool, post_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET view_count = view_count + 1
            WHERE post_id = $1
            "#,
            post_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
