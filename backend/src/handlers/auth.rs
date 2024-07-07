use crate::models::user_model::User;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
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
                    return HttpResponse::Ok().json(user); // Return user info
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
