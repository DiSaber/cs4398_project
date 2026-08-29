mod app_state;
mod database;
mod models;
mod routes;

use axum::Router;

use crate::{app_state::AppState, database::Database, routes::lobbies};

#[tokio::main]
async fn main() {
    let database = Database::connect()
        .await
        .expect("Database should be connected");
    database
        .create_tables()
        .await
        .expect("Database tables should be created");

    let app = Router::new()
        .nest("/api/lobbies", lobbies::router())
        .with_state(AppState { database });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
