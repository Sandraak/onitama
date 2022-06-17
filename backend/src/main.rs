use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_session::MemoryStore;
use axum::extract::{Path, WebSocketUpgrade};
use axum::{
    extract::Extension,
    http::{self, header::HeaderMap},
    response::IntoResponse,
    routing::get,
    Json, Router, Server,
};
use tracing::debug;
use uuid::Uuid;

use onitama::card::Card;
use onitama::State as GameState;

use crate::session::{UserId, UserIdFromSession, AXUM_SESSION_COOKIE_NAME};

mod session;

pub struct Game {
    p1: UserId,
    p2: Option<UserId>,
    state: GameState,
}

pub type Database = Arc<Mutex<HashMap<Uuid, Game>>>;

async fn create(user_id: UserIdFromSession, Extension(db): Extension<Database>) -> Json<Uuid> {
    let game = Game {
        p1: user_id.into(),
        p2: None,
        state: GameState::new(),
    };
    let key = Uuid::new_v4();

    db.lock().unwrap().insert(key, game);
    Json(key)
}

async fn connect(
    user_id: UserIdFromSession,
    Extension(db): Extension<Database>,
    Path(game_id): Path<Uuid>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move { while let Some(x) = socket.recv().await {} })
}

async fn card() -> Json<Card> {
    let card = GameState::new().spare_card.clone();
    Json(card)
}

async fn handler(user_id: UserIdFromSession) -> impl IntoResponse {
    let (headers, user_id, create_cookie) = match user_id {
        UserIdFromSession::FoundUserId(user_id) => (HeaderMap::new(), user_id, false),
        UserIdFromSession::CreatedFreshUserId(new_user) => {
            let mut headers = HeaderMap::new();
            headers.insert(http::header::SET_COOKIE, new_user.cookie);
            (headers, new_user.user_id, true)
        }
    };

    debug!("handler: user_id={:?} send_headers={:?}", user_id, headers);

    (
        headers,
        format!(
            "user_id={:?} session_cookie_name={} create_new_session_cookie={}",
            user_id, AXUM_SESSION_COOKIE_NAME, create_cookie
        ),
    )
}

const HOST: &str = env!("HOST");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db: Database = Arc::new(Mutex::new(HashMap::new()));
    let store = MemoryStore::new();
    let addr = HOST.parse().expect("invalid host");
    let app = Router::new()
        .route("/api", get(handler))
        .route("/api/card", get(card))
        .route("/api/create", get(create))
        .route("/api/connect/:game_id", get(connect))
        .layer(Extension(store))
        .layer(Extension(db));

    debug!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
