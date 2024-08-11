use crate::models::friend_model::Friend;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn add_friend_request(
    pool: web::Data<PgPool>,
    user_id1: web::Path<i32>,
    user_id2: web::Path<i32>,
) -> impl Responder {
    match Friend::add_friend_request(pool.get_ref(), *user_id1, *user_id2).await {
        Ok(friend_request) => HttpResponse::Ok().json(friend_request),
        Err(e) => {
            eprintln!("Error adding friend request: {:?}", e);
            HttpResponse::InternalServerError().json("Error adding friend request")
        }
    }
}

pub async fn confirm_friend_request(
    pool: web::Data<PgPool>,
    user_id1: web::Path<i32>,
    user_id2: web::Path<i32>,
) -> impl Responder {
    match Friend::confirm_friend_request(pool.get_ref(), *user_id1, *user_id2).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                HttpResponse::Ok().json("Friend request confirmed")
            } else {
                HttpResponse::BadRequest().json("No friend request found")
            }
        }
        Err(e) => {
            eprintln!("Error confirming friend request: {:?}", e);
            HttpResponse::InternalServerError().json("Error confirming friend request")
        }
    }
}

pub async fn delete_friend(
    pool: web::Data<PgPool>,
    user_id1: web::Path<i32>,
    user_id2: web::Path<i32>,
) -> impl Responder {
    match Friend::delete_friend(pool.get_ref(), *user_id1, *user_id2).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                HttpResponse::Ok().json("Friend deleted")
            } else {
                HttpResponse::BadRequest().json("No friendship found")
            }
        }
        Err(e) => {
            eprintln!("Error deleting friend: {:?}", e);
            HttpResponse::InternalServerError().json("Error deleting friend")
        }
    }
}

pub async fn get_friends(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    match Friend::get_friends(pool.get_ref(), *user_id).await {
        Ok(friends) => HttpResponse::Ok().json(friends),
        Err(e) => {
            eprintln!("Error fetching friends: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching friends")
        }
    }
}

pub async fn are_friends(
    pool: web::Data<PgPool>,
    user_id1: web::Path<i32>,
    user_id2: web::Path<i32>,
) -> impl Responder {
    match Friend::are_friends(pool.get_ref(), *user_id1, *user_id2).await {
        Ok(are_friends) => {
            if are_friends {
                HttpResponse::Ok().json("Users are friends")
            } else {
                HttpResponse::Ok().json("Users are not friends")
            }
        }
        Err(e) => {
            eprintln!("Error checking friendship: {:?}", e);
            HttpResponse::InternalServerError().json("Error checking friendship")
        }
    }
}
