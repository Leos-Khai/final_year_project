use crate::models::post_model::Post;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn create_post(
    pool: web::Data<PgPool>,
    session: Session,
    new_post: web::Json<Post>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let mut post = new_post.into_inner();
    post.author_id = user_id;

    match Post::create(pool.get_ref(), post).await {
        Ok(created_post) => HttpResponse::Ok().json(created_post),
        Err(e) => {
            eprintln!("Error creating post: {:?}", e);
            HttpResponse::InternalServerError().json("Error creating post")
        }
    }
}

pub async fn get_post_by_id(pool: web::Data<PgPool>, post_id: web::Path<i32>) -> impl Responder {
    match Post::find_by_id(pool.get_ref(), *post_id).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching post")
        }
    }
}

pub async fn get_posts_by_user_id(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Post,
        r#"
        SELECT post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id
        FROM posts
        WHERE author_id = $1
        "#,
        *user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching posts")
        }
    }
}

pub async fn get_posts_by_friends(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let result = sqlx::query_as!(
        Post,
        r#"
        SELECT p.post_id, p.post_title, p.post_content, p.post_date, p.like_count, p.view_count, p.author_type, p.author_id
        FROM posts p
        INNER JOIN friends f ON p.author_id = f.user_id1
        WHERE f.user_id1 = $1
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching posts")
        }
    }
}

pub async fn get_all_posts(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Post,
        r#"
        SELECT post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id
        FROM posts
        ORDER BY post_date DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching posts")
        }
    }
}
