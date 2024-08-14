use crate::fake_news_detector::FakeNewsDetector;
use crate::models::post_model::Post;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn create_post(
    pool: web::Data<PgPool>,
    session: Session,
    new_post: web::Json<Post>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let user_type: String = match session.get("user_type") {
        Ok(Some(user_type)) => user_type,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let mut post = new_post.into_inner();
    post.author_id = user_id;
    post.author_type = user_type;

    println!("Incoming post data: {:?}", post);

    match Post::create(pool.get_ref(), post).await {
        Ok(created_post) => HttpResponse::Ok().json(created_post),
        Err(e) => {
            eprintln!("Error creating post: {:?}", e);
            HttpResponse::InternalServerError().json("Error creating post")
        }
    }
}

pub async fn get_post_by_id(pool: web::Data<PgPool>, post_id: web::Path<i32>) -> impl Responder {
    match Post::increment_view_count(pool.get_ref(), *post_id).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error incrementing view count: {:?}", e);
            return HttpResponse::InternalServerError().json("Error incrementing view count");
        }
    }

    match Post::find_by_id(pool.get_ref(), *post_id).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching post")
        }
    }
}

pub async fn get_posts_by_user_id(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Post,
        r#"
        SELECT post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id,
               COALESCE(
                   CASE
                       WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                       WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                   END, NULL
               ) AS author_name
        FROM posts
        WHERE author_id = $1
        "#,
        *user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching posts")
        }
    }
}

pub async fn get_posts_by_friends(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    let result = sqlx::query_as!(
        Post,
        r#"
        SELECT p.post_id, p.post_title, p.post_content, p.post_date, p.like_count, p.view_count, p.author_type, p.author_id,
               COALESCE(
                   CASE
                       WHEN p.author_type = 'member' THEN (SELECT username FROM member WHERE member_id = p.author_id)
                       WHEN p.author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = p.author_id)
                   END, NULL
               ) AS author_name
        FROM posts p
        INNER JOIN friends f ON p.author_id = f.user_id1
        WHERE f.user_id1 = $1
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching posts")
        }
    }
}

pub async fn get_all_posts(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Post,
        r#"
        SELECT post_id, post_title, post_content, post_date, like_count, view_count, author_type, author_id,
               COALESCE(
                   CASE
                       WHEN author_type = 'member' THEN (SELECT username FROM member WHERE member_id = author_id)
                       WHEN author_type = 'admin' THEN (SELECT username FROM admin WHERE admin_id = author_id)
                   END, NULL
               ) AS author_name
        FROM posts
        ORDER BY post_date DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error fetching posts")
        }
    }
}

pub async fn update_post(
    pool: web::Data<PgPool>,
    session: Session,
    post_id: web::Path<i32>,
    updated_post: web::Json<Post>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    println!("Post: {:?}", updated_post);

    // Fetch the existing post to verify the author
    let existing_post = match Post::find_by_id(pool.get_ref(), *post_id).await {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            return HttpResponse::InternalServerError().json("Error fetching post");
        }
    };

    if existing_post.author_id != user_id {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }

    let updated_post = updated_post.into_inner();

    match Post::update(pool.get_ref(), *post_id, updated_post).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            eprintln!("Error updating post: {:?}", e);
            HttpResponse::InternalServerError().json("Error updating post")
        }
    }
}

pub async fn delete_post(
    pool: web::Data<PgPool>,
    session: Session,
    post_id: web::Path<i32>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    // Fetch the existing post to verify the author
    let existing_post = match Post::find_by_id(pool.get_ref(), *post_id).await {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            return HttpResponse::InternalServerError().json("Error fetching post");
        }
    };

    if existing_post.author_id != user_id {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }

    match Post::delete(pool.get_ref(), *post_id).await {
        Ok(_) => HttpResponse::Ok().json("Post deleted"),
        Err(e) => {
            eprintln!("Error deleting post: {:?}", e);
            HttpResponse::InternalServerError().json("Error deleting post")
        }
    }
}

pub async fn like_post(
    pool: web::Data<PgPool>,
    session: Session,
    post_id: web::Path<i32>,
) -> impl Responder {
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    match Post::like_post(pool.get_ref(), user_id, *post_id).await {
        Ok(updated_post) => HttpResponse::Ok().json(updated_post),
        Err(e) => {
            eprintln!("Error liking post: {:?}", e);
            HttpResponse::InternalServerError().json("Error liking post")
        }
    }
}

pub async fn check_post_validity(
    pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    // Fetch the post by ID
    let post = match Post::find_by_id(pool.get_ref(), *post_id).await {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            return HttpResponse::InternalServerError().json("Error fetching post");
        }
    };

    // Initialize the fake news detector
    let mut detector = match FakeNewsDetector::new("bert_fake_news_detector.onnx", "tokenizer.json")
    {
        Ok(detector) => detector,
        Err(e) => {
            eprintln!("Error initializing fake news detector: {:?}", e);
            return HttpResponse::InternalServerError()
                .json("Error initializing fake news detector");
        }
    };

    // Validate the post for fake news
    match detector.validate_post(&post) {
        Ok((result, fake_prob, real_prob)) => {
            println!(
                "Fake news detection result: {}, Fake: {}, Real: {}",
                result, fake_prob, real_prob
            );
            HttpResponse::Ok().json(serde_json::json!({
                "result": result,
                "fake_probability": fake_prob,
                "real_probability": real_prob
            }))
        }
        Err(e) => {
            eprintln!("Error detecting fake news: {:?}", e);
            HttpResponse::InternalServerError().json("Error detecting fake news")
        }
    }
}
