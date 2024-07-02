use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterInput {
    username: String,
    password: String,
}

async fn register(info: web::Json<RegisterInput>) -> impl Responder {
    // Implement user registration logic here
    HttpResponse::Ok().json("User registered")
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

async fn login(info: web::Json<LoginInput>) -> impl Responder {
    // Implement user login logic here
    HttpResponse::Ok().json("User logged in")
}
