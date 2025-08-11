use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::migrate::Migrator;
use sqlx::Executor;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use trombone::app_state::AppState;
use trombone::auth::Claims;
use trombone::{db::setup_database_pool, router::router};

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn setup() -> (axum::Router, String) {
    dotenvy::dotenv().ok();
    let pool = setup_database_pool().await;

    // Apply migrations
    MIGRATOR.run(&pool).await.unwrap();

    // Run the seed script
    let seed_sql = fs::read_to_string(Path::new("tests/seed.sql")).unwrap();
    pool.execute(seed_sql.as_str()).await.unwrap();

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set for tests");

    let app_state = AppState {
        db_pool: pool,
        jwt_secret: jwt_secret.clone(),
    };

    // Create a test user
    let user_id = Uuid::new_v4();
    let hashed_password = bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap();
    let email = format!("test.user+{}@example.com", Uuid::new_v4());
    sqlx::query!(
        r#"
        INSERT INTO users (id, first_name, last_name, email, password_hash)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        user_id,
        "Test",
        "User",
        email,
        hashed_password
    )
    .execute(&app_state.db_pool)
    .await
    .unwrap();

    // Generate JWT for the test user
    let claims = Claims {
        sub: user_id,
        exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize, // Token expires in 1 hour
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    (router(app_state), token)
}
