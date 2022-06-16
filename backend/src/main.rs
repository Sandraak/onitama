use axum::{Json, Router, Server};
use axum::routing::get;
use tracing::debug;
use onitama::card::Card;

const HOST: &str = env!("HOST");

async fn index() -> Json<Card> {
    let card = Card::deck().pop().unwrap();
    Json(card)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = HOST.parse().expect("invalid host");
    let app = Router::new()
        .route("/api", get(index));

    debug!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
