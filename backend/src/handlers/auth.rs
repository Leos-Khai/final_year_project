use crate::email::send_reset_email;
use crate::models::password_reset_token_model::PasswordResetToken;
use crate::models::user_model::User;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub user_type: String,
}

pub async fn register(pool: web::Data<PgPool>, info: web::Json<RegisterInput>) -> impl Responder {
    println!("Received register request: {:?}", info);

    let username = &info.username;
    let email = &info.email;
    let password = &info.password;

    // Hash the password
    let password_hash = match hash(password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error hashing password: {:?}", e);
            return HttpResponse::InternalServerError().json("Error hashing password");
        }
    };

    // Create a new member
    let new_member = crate::models::member_model::Member {
        member_id: 0, // This will be ignored by the database
        username: username.clone(),
        email: email.clone(),
        password_hash: password_hash.clone(),
        full_name: None,
        phone_number: None,
        profile_pic: None,
    };

    match crate::models::member_model::Member::create(&pool, new_member).await {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json("Error creating user")
        }
    }
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub async fn login(
    pool: web::Data<PgPool>,
    info: web::Json<LoginInput>,
    session: Session,
) -> impl Responder {
    let username = &info.username;
    let password = &info.password;

    let result = User::find_by_username(pool.get_ref(), username).await;

    match result {
        Ok(user) => {
            if let Some(ref password_hash) = user.password_hash {
                if verify(password, password_hash).unwrap() {
                    session.insert("user_id", user.user_id).unwrap();
                    session.insert("user_type", user.user_type.clone()).unwrap();
                    println!("User logged in: {:?}", user); // Print user info on login

                    // Create a UserResponse instance
                    let user_response = UserResponse {
                        user_id: user.user_id.unwrap(),
                        username: user.username.unwrap(),
                        user_type: user.user_type.unwrap(),
                    };

                    return HttpResponse::Ok().json(user_response); // Return user info without password hash
                } else {
                    println!("Failed login attempt for username: {}", username);
                    return HttpResponse::Unauthorized().json("Invalid credentials");
                }
            } else {
                println!("Failed login attempt for username: {}", username);
                return HttpResponse::Unauthorized().json("Invalid credentials");
            }
        }
        Err(_) => {
            println!("Failed login attempt for username: {}", username);
            return HttpResponse::Unauthorized().json("Invalid credentials");
        }
    }
}

pub async fn logout(session: Session) -> impl Responder {
    if let Some(user_id) = session.get::<i32>("user_id").unwrap_or(None) {
        println!("User logged out: user_id {}", user_id); // Print user info on logout
        session.purge();
        HttpResponse::Ok().json("Logged out")
    } else {
        HttpResponse::Unauthorized().json("No user logged in")
    }
}

#[derive(Deserialize)]
pub struct ResetEmailInput {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordInput {
    pub token: String,
    pub new_password: String,
}

pub async fn request_password_reset(
    pool: web::Data<PgPool>,
    info: web::Json<ResetEmailInput>,
) -> impl Responder {
    let email = &info.email;

    match User::find_by_email(pool.get_ref(), email).await {
        Ok(user) => {
            let token = Uuid::new_v4().to_string();
            let expires = (Utc::now() + Duration::hours(1)).naive_utc();
            PasswordResetToken::create(pool.get_ref(), user.user_id.unwrap(), &token, expires)
                .await
                .unwrap();
            send_reset_email(email, &token);
            HttpResponse::Ok().json("Password reset email sent")
        }
        Err(_) => HttpResponse::NotFound().json("Email not found"),
    }
}

pub async fn reset_password(
    pool: web::Data<PgPool>,
    info: web::Json<ResetPasswordInput>,
) -> impl Responder {
    let token = &info.token;
    let new_password = &info.new_password;

    match PasswordResetToken::find_by_token(pool.get_ref(), token).await {
        Ok(reset_token) => {
            if Utc::now().naive_utc() > reset_token.reset_token_expires {
                return HttpResponse::BadRequest().json("Token has expired");
            }

            let password_hash = hash(new_password, bcrypt::DEFAULT_COST).unwrap();
            User::update_password(pool.get_ref(), reset_token.user_id, &password_hash)
                .await
                .unwrap();
            PasswordResetToken::delete_by_user_id(pool.get_ref(), reset_token.user_id)
                .await
                .unwrap();
            HttpResponse::Ok().json("Password has been reset")
        }
        Err(_) => HttpResponse::BadRequest().json("Invalid token"),
    }
}
