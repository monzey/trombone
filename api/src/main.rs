use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use trombone::{app_state::AppState, db, router::router};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_pool = db::setup_database_pool().await;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = AppState {
        db_pool,
        jwt_secret,
    };

    let app = router(app_state).layer(CorsLayer::very_permissive());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

