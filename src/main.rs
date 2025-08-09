use std::net::SocketAddr;

use trombone::{db, router::router};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_pool = db::setup_database_pool().await;
    let app = router(db_pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333)); // TODO: Make this configurable
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
