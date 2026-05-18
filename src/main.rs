mod app;
mod entity;
mod model;
mod repository;
mod router;
mod validate;

use sqlx::{migrate, postgres::PgPoolOptions};
use tokio::{net::TcpListener, signal};

use crate::app::build_app;

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    println!("database connected.");

    migrate!().run(&pool).await.unwrap();
    println!("migration complated.");

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    let app = build_app(pool);

    println!("listenning on {}.", addr);
    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
}

async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.unwrap() };
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
