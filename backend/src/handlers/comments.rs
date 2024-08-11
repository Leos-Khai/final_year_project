use crate::models::comment_model::Comment;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCommentPayload {
    pub post_id: i32,
    pub comment_content: String,
}

pub async fn create_comment(
    pool: web::Data<PgPool>,
    session: Session,
    new_comment: web::Json<CreateCommentPayload>, // Adjusted the expected payload
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let user_type: String = match session.get("user_type") {
        Ok(Some(user_type)) => user_type,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    println!("Received new comment: {:?}", new_comment);

    let comment = Comment {
        author_id: user_id,
        author_type: user_type,
        comment_content: new_comment.comment_content.clone(),
        post_id: new_comment.post_id,
        author_name: "khai".to_string(),
        comment_date: None,
        comment_id: 0,
    };

    match Comment::create(pool.get_ref(), comment).await {
        Ok(created_comment) => HttpResponse::Ok().json(created_comment),
        Err(e) => {
            eprintln!("Error creating comment: {:?}", e);
            HttpResponse::InternalServerError().json("Error creating comment")
        }
    }
}

pub async fn get_comment_by_id(
    pool: web::Data<PgPool>,
    comment_id: web::Path<i32>,
) -> impl Responder {
    match Comment::find_by_id(pool.get_ref(), *comment_id).await {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(e) => {
            eprintln!("Error fetching comment: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching comment")
        }
    }
}

pub async fn get_comments_by_post_id(
    pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    match Comment::find_by_post_id(pool.get_ref(), *post_id).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            eprintln!("Error fetching comments: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching comments")
        }
    }
}

pub async fn update_comment(
    pool: web::Data<PgPool>,
    session: Session,
    comment_id: web::Path<i32>,
    updated_content: web::Json<String>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    // Fetch the existing comment to verify the author
    let existing_comment = match Comment::find_by_id(pool.get_ref(), *comment_id).await {
        Ok(comment) => comment,
        Err(e) => {
            eprintln!("Error fetching comment: {:?}", e);
            return HttpResponse::InternalServerError().json("Error fetching comment");
        }
    };

    if existing_comment.author_id != user_id {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }

    // Update the comment using the method from the model
    match Comment::update(pool.get_ref(), *comment_id, updated_content.into_inner()).await {
        Ok(updated_comment) => HttpResponse::Ok().json(updated_comment),
        Err(e) => {
            eprintln!("Error updating comment: {:?}", e);
            HttpResponse::InternalServerError().json("Error updating comment")
        }
    }
}

pub async fn delete_comment(
    pool: web::Data<PgPool>,
    session: Session,
    comment_id: web::Path<i32>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    // Fetch the existing comment to verify the author
    let existing_comment = match Comment::find_by_id(pool.get_ref(), *comment_id).await {
        Ok(comment) => comment,
        Err(e) => {
            eprintln!("Error fetching comment: {:?}", e);
            return HttpResponse::InternalServerError().json("Error fetching comment");
        }
    };

    if existing_comment.author_id != user_id {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }

    // Delete the comment using the method from the model
    match Comment::delete(pool.get_ref(), *comment_id).await {
        Ok(_) => HttpResponse::Ok().json("Comment deleted"),
        Err(e) => {
            eprintln!("Error deleting comment: {:?}", e);
            HttpResponse::InternalServerError().json("Error deleting comment")
        }
    }
}
