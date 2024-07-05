use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::member_model::Member;

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(pool: web::Data<PgPool>, info: web::Json<RegisterInput>) -> impl Responder {
    let username = &info.username;
    let email = &info.email;
    let password = &info.password;

    // Hash the password
    let password_hash = match hash(password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().json("Error hashing password"),
    };

    // Create a new member
    let new_member = Member {
        member_id: 0, // This will be ignored by the database
        username: username.clone(),
        email: email.clone(),
        password_hash: password_hash.clone(),
        full_name: None,
        phone_number: None,
        profile_pic: None,
    };

    match Member::create(&pool, new_member).await {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub async fn login(info: web::Json<LoginInput>) -> impl Responder {
    let username = &info.username;
    let password = &info.password;
    HttpResponse::Ok().json(format!(
        "User {} logged in with password {}",
        username, password
    ))
}
