mod handlers;
mod models;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Create a key for the cookie session
    let private_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials(); // This line allows credentials to be sent

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                private_key.clone(),
            ))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/logout", web::post().to(handlers::auth::logout)),
            )
            .service(
                web::scope("/posts")
                    .route("/create", web::post().to(handlers::posts::create_post))
                    .route("/{id}", web::get().to(handlers::posts::get_post_by_id))
                    .route(
                        "/user/{user_id}",
                        web::get().to(handlers::posts::get_posts_by_user_id),
                    )
                    .route(
                        "/friends",
                        web::get().to(handlers::posts::get_posts_by_friends),
                    )
                    .route("/all", web::get().to(handlers::posts::get_all_posts)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
