use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct CommentInput {
    content: String,
}

async fn add_comment(post_id: web::Path<String>, info: web::Json<CommentInput>) -> impl Responder {
    // Implement comment creation logic here
    HttpResponse::Ok().json("Comment added")
}
