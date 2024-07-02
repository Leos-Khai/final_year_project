use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct PostInput {
    content: String,
}

async fn create_post(info: web::Json<PostInput>) -> impl Responder {
    // Implement post creation logic here
    HttpResponse::Ok().json("Post created")
}

async fn get_posts() -> impl Responder {
    // Implement logic to get all posts here
    HttpResponse::Ok().json("All posts")
}
