mod email;
mod fake_news_detector;
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
                    .route("/logout", web::post().to(handlers::auth::logout))
                    .route(
                        "/request-reset",
                        web::post().to(handlers::auth::request_password_reset),
                    )
                    .route(
                        "/reset-password",
                        web::post().to(handlers::auth::reset_password),
                    ),
            )
            .service(
                web::scope("/posts")
                    .route("/all", web::get().to(handlers::posts::get_all_posts))
                    .route(
                        "/check-validity/{id}",
                        web::get().to(handlers::posts::check_post_validity),
                    )
                    .route("/create", web::post().to(handlers::posts::create_post))
                    .route(
                        "/user/{user_id}",
                        web::get().to(handlers::posts::get_posts_by_user_id),
                    )
                    .route(
                        "/friends",
                        web::get().to(handlers::posts::get_posts_by_friends),
                    )
                    .route("/like/{id}", web::post().to(handlers::posts::like_post))
                    .route("/{id}", web::get().to(handlers::posts::get_post_by_id))
                    .route("/{id}", web::put().to(handlers::posts::update_post))
                    .route("/{id}", web::delete().to(handlers::posts::delete_post)),
            )
            .service(
                web::scope("/friends")
                    .route(
                        "/add",
                        web::post().to(handlers::friends::add_friend_request),
                    )
                    .route(
                        "/confirm",
                        web::post().to(handlers::friends::confirm_friend_request),
                    )
                    .route(
                        "/delete",
                        web::delete().to(handlers::friends::delete_friend),
                    )
                    .route(
                        "/{user_id}/list",
                        web::get().to(handlers::friends::get_friends),
                    )
                    .route("/check", web::get().to(handlers::friends::are_friends)),
            )
            .service(
                web::scope("/comments")
                    .route(
                        "/create",
                        web::post().to(handlers::comments::create_comment),
                    )
                    .route(
                        "/{id}",
                        web::get().to(handlers::comments::get_comment_by_id),
                    )
                    .route(
                        "/post/{post_id}",
                        web::get().to(handlers::comments::get_comments_by_post_id),
                    )
                    .route("/{id}", web::put().to(handlers::comments::update_comment)) // New route for updating a comment
                    .route(
                        "/{id}",
                        web::delete().to(handlers::comments::delete_comment),
                    ),
            )
            .service(
                web::scope("/profile") // New scope for user profile management
                    .route("/", web::get().to(handlers::user::get_profile)) // Route for getting the user's profile
                    .route("/", web::put().to(handlers::user::update_profile)) // Route for updating the user's profile
                    .route("/", web::delete().to(handlers::user::delete_profile)) // Route for deleting the user's profile
                    .route(
                        "/update-password",
                        web::put().to(handlers::user::update_password),
                    ), // New route for updating password
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
