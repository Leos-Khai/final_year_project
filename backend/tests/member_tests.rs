use backend::models::member_model::Member;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tokio;
async fn get_test_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

#[tokio::test]
async fn test_create_member() {
    let pool = get_test_pool().await;

    let new_member = Member {
        member_id: 0, // Will be set by the database
        username: String::from("testuser"),
        email: String::from("test@example.com"),
        password_hash: String::from("hashedpassword"),
        full_name: Some(String::from("Test User")),
        phone_number: Some(String::from("1234567890")),
        profile_pic: None,
    };

    let created_member = Member::create(&pool, new_member)
        .await
        .expect("Failed to create member");
    assert_eq!(created_member.username, "testuser");
}

#[tokio::test]
async fn test_get_member_by_id() {
    let pool = get_test_pool().await;

    let new_member = Member {
        member_id: 0, // Will be set by the database
        username: String::from("testuser2"),
        email: String::from("test2@example.com"),
        password_hash: String::from("hashedpassword2"),
        full_name: Some(String::from("Test User2")),
        phone_number: Some(String::from("1234567891")),
        profile_pic: None,
    };

    let created_member = Member::create(&pool, new_member)
        .await
        .expect("Failed to create member");

    let fetched_member = Member::get_by_id(&pool, created_member.member_id)
        .await
        .expect("Failed to fetch member");
    assert_eq!(fetched_member.username, "testuser2");
}

#[tokio::test]
async fn test_update_member() {
    let pool = get_test_pool().await;

    let new_member = Member {
        member_id: 0, // Will be set by the database
        username: String::from("testuser3"),
        email: String::from("test3@example.com"),
        password_hash: String::from("hashedpassword3"),
        full_name: Some(String::from("Test User3")),
        phone_number: Some(String::from("1234567892")),
        profile_pic: None,
    };

    let created_member = Member::create(&pool, new_member)
        .await
        .expect("Failed to create member");

    let updated_member = Member {
        username: String::from("updateduser"),
        email: String::from("updated@example.com"),
        password_hash: String::from("updatedpassword"),
        full_name: Some(String::from("Updated User")),
        phone_number: Some(String::from("9876543210")),
        profile_pic: Some(String::from("updatedpic.jpg")),
        ..created_member
    };

    let updated_member = Member::update(&pool, created_member.member_id, updated_member)
        .await
        .expect("Failed to update member");

    assert_eq!(updated_member.username, "updateduser");
    assert_eq!(updated_member.email, "updated@example.com");
    assert_eq!(updated_member.full_name.unwrap(), "Updated User");
    assert_eq!(updated_member.phone_number.unwrap(), "9876543210");
    assert_eq!(updated_member.profile_pic.unwrap(), "updatedpic.jpg");
}

#[tokio::test]
async fn test_delete_member() {
    let pool = get_test_pool().await;

    let new_member = Member {
        member_id: 0, // Will be set by the database
        username: String::from("testuser4"),
        email: String::from("test4@example.com"),
        password_hash: String::from("hashedpassword4"),
        full_name: Some(String::from("Test User4")),
        phone_number: Some(String::from("1234567893")),
        profile_pic: None,
    };

    let created_member = Member::create(&pool, new_member)
        .await
        .expect("Failed to create member");

    let deleted_rows = Member::delete(&pool, created_member.member_id)
        .await
        .expect("Failed to delete member");

    assert_eq!(deleted_rows, 1);
}
