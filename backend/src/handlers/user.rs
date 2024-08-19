use crate::models::member_model::Member;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

/// Get the logged-in user's profile
pub async fn get_profile(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    match Member::get_by_id(pool.get_ref(), user_id).await {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(e) => {
            eprintln!("Error fetching profile: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching profile")
        }
    }
}

/// Update the logged-in user's profile
pub async fn update_profile(
    pool: web::Data<PgPool>,
    session: Session,
    updated_member: web::Json<Member>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    // Fetch the existing member to check ownership
    let existing_member = match Member::get_by_id(pool.get_ref(), user_id).await {
        Ok(member) => member,
        Err(e) => {
            eprintln!("Error fetching profile: {:?}", e);
            return HttpResponse::InternalServerError().json("Error fetching profile");
        }
    };
    println!("pass 1");

    if existing_member.member_id != user_id {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }
    println!("pass 2");
    // Update the profile with new data
    let updated_member = Member {
        member_id: user_id, // Ensure we are updating the correct user
        ..updated_member.into_inner()
    };

    match Member::update(pool.get_ref(), user_id, updated_member).await {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(e) => {
            eprintln!("Error updating profile: {:?}", e);
            HttpResponse::InternalServerError().json("Error updating profile")
        }
    }
}

/// Delete the logged-in user's profile
pub async fn delete_profile(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    match Member::delete(pool.get_ref(), user_id).await {
        Ok(_) => HttpResponse::Ok().json("Profile deleted"),
        Err(e) => {
            eprintln!("Error deleting profile: {:?}", e);
            HttpResponse::InternalServerError().json("Error deleting profile")
        }
    }
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    old_password: String,
    new_password: String,
}

/// Update the logged-in user's password
pub async fn update_password(
    pool: web::Data<PgPool>,
    session: Session,
    passwords: web::Json<UpdatePasswordRequest>,
) -> impl Responder {
    // Extract the user ID from the session
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let UpdatePasswordRequest {
        old_password,
        new_password,
    } = passwords.into_inner();
    println!("pass password 1");
    // Call the update_password method from the Member model
    match Member::update_password(pool.get_ref(), user_id, &old_password, &new_password).await {
        Ok(_) => HttpResponse::Ok().json("Password updated successfully"),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::Unauthorized().json("Old password is incorrect")
        }
        Err(_) => HttpResponse::InternalServerError().json("Error updating password"),
    }
}
