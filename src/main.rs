mod app;
mod auth;
mod entity;
mod error;
mod model;
mod repository;
mod router;
mod validate;

use sqlx::{migrate, postgres::PgPoolOptions};
use tokio::{net::TcpListener, signal};

use crate::app::{Config, build_app};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let pool = PgPoolOptions::new().connect(&config.db_url).await.unwrap();
    println!("database connected.");

    migrate!().run(&pool).await.unwrap();
    println!("migration complated.");

    let listener = TcpListener::bind(&config.addr).await.unwrap();
    println!("listenning on {}.", &config.addr);

    let app = build_app(pool, config);
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
