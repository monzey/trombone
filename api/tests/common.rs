use sqlx::migrate::Migrator;
use sqlx::Executor;
use std::fs;
use std::path::Path;

use trombone::app_state::AppState;
use trombone::{db::setup_database_pool, router::router};

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn setup() -> axum::Router {
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
        jwt_secret,
    };

    router(app_state)
}

