use axum::{routing::get, Extension, Json, Router};
use sqlx::{PgPool, Row};
use std::net::SocketAddr;

mod config;

#[tokio::main]
async fn main() {
    let pool = config::db::connect().await;

    let app = Router::new().merge(route()).layer(Extension(pool));

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server started, listening on http://{addr}");
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

fn route() -> Router {
    let router = Router::new().route("/", get(handler));
    router
}

async fn handler(pool: axum::extract::Extension<PgPool>) -> Json<Message> {
    let row = sqlx::query("SELECT * FROM messages")
        .fetch_one(&*pool)
        .await
        .expect("Failed to fetch message from database");

    let message: String = row.try_get("message").unwrap_or_default();

    Json(Message { message })
}
