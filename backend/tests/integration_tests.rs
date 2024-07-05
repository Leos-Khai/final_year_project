use actix_web::{test, web, App};
use backend::handlers::auth::{login, register}; // Ensure this matches your crate name and module structure

#[actix_rt::test]
async fn test_register() {
    let mut app = test::init_service(
        App::new().service(web::scope("/auth").route("/register", web::post().to(register))),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&serde_json::json!({
            "username": "testuser",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_login() {
    let mut app = test::init_service(
        App::new().service(web::scope("/auth").route("/login", web::post().to(login))),
    )
    .await;

    // Register user first
    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&serde_json::json!({
            "username": "testuser",
            "password": "password123"
        }))
        .to_request();
    let _ = test::call_service(&mut app, req).await;

    // Now login
    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&serde_json::json!({
            "username": "testuser",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
