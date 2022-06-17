use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_session::MemoryStore;
use axum::extract::ws::Message;
use axum::extract::{Path, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::{
    extract::Extension,
    http::{self, header::HeaderMap},
    response::IntoResponse,
    routing::get,
    Json, Router, Server,
};
use tracing::debug;
use uuid::Uuid;

use onitama::card::{Card, Move};
use onitama::{Colour, MovePiece, State as GameState};

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

async fn submit(
    user_id: UserIdFromSession,
    Extension(db): Extension<Database>,
    Path(game_id): Path<Uuid>,
    Path(mov): Path<MovePiece>,
) -> Result<String, HandleError> {
    let user_id = user_id.into();
    let mut guard = db.lock().unwrap();
    let game = guard.get_mut(&game_id).ok_or(HandleError::NoGame)?;

    let mut succes_string = "Prima.".to_string();

    if game.p2.is_none() {
        return Err(HandleError::NoSecondPlayer);
    }
    let current_player = game.state.current_player();
    if (game.p1 == user_id && current_player == Colour::Red)
        || (game.p2 == Some(user_id) && current_player == Colour::Blue)
    {
        if game.state.perform_mov(mov).is_err() {
            return Err(HandleError::InvalidMove);
        }
    } else {
        return Err(HandleError::OutOfTurn);
    }

    if game.state.winner().is_some() {
        if game.state.winner().unwrap() == Colour::Red {
            succes_string = "Red won".to_string();
        } else {
            succes_string = "Blue won".to_string();
        }
    }
    Ok(succes_string)
}

async fn connect(
    user_id: UserIdFromSession,
    Extension(db): Extension<Database>,
    Path(game_id): Path<Uuid>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, HandleError> {
    if db.lock().unwrap().get(&game_id).is_none() {
        return Err(HandleError::NoGame);
    }
    Ok(ws.on_upgrade(move |mut socket| async move {
        let user_id = user_id.into();
        while db.lock().unwrap().get(&game_id).unwrap().p2.is_none() {
            tokio::time::sleep(tokio::time::Duration::from_millis(500));
            socket.send(Message::Text("Waiting for player 2...".to_string()));
        }

        while let Some(Ok(message)) = socket.recv().await {
            // Wat als dit geen move is?
            let mov: MovePiece = serde_json::from_str(&message.into_text().unwrap()).unwrap();
            //   let game = db.lock().unwrap().get_mut(&game_id).unwrap();
            let current_player = db
                .lock()
                .unwrap()
                .get_mut(&game_id)
                .unwrap()
                .state
                .current_player();
            if (db.lock().unwrap().get_mut(&game_id).unwrap().p1 == user_id
                && current_player == Colour::Red)
                || (db.lock().unwrap().get_mut(&game_id).unwrap().p2 == Some(user_id)
                    && current_player == Colour::Blue)
            {
                if db
                    .lock()
                    .unwrap()
                    .get_mut(&game_id)
                    .unwrap()
                    .state
                    .perform_mov(mov)
                    .is_err()
                {
                    socket.send(Message::Text("Invalid move by player 1.".to_string()));
                } else {
                    socket.send(Message::Text("Prima p1".to_string()));
                }
            }
            if db
                .lock()
                .unwrap()
                .get_mut(&game_id)
                .unwrap()
                .state
                .winner()
                .is_some()
            {
                if db
                    .lock()
                    .unwrap()
                    .get_mut(&game_id)
                    .unwrap()
                    .state
                    .winner()
                    .unwrap()
                    == Colour::Red
                {
                    socket.send(Message::Text("Red won".to_string()));
                } else {
                    socket.send(Message::Text("Blue won".to_string()));
                }
            }
        }
    }))
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
        .route("/api/submit/:game_id/:mov", get(submit))
        .layer(Extension(store))
        .layer(Extension(db));

    debug!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

enum HandleError {
    NoGame,
    NoSecondPlayer,
    InvalidMove,
    OutOfTurn,
}

impl IntoResponse for HandleError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            HandleError::NoGame => "No game id found.",
            HandleError::NoSecondPlayer => "Wait for second player.",
            HandleError::InvalidMove => "Invalid move.",
            HandleError::OutOfTurn => "Out of turn.",
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
