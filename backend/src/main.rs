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
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use onitama::card::{Card, Shift};
use onitama::{Action, State as GameState, Team};

use crate::session::{UserId, UserIdFromSession, AXUM_SESSION_COOKIE_NAME};

mod session;

#[derive(Clone, Serialize)]
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
    Path((game_id, mov)): Path<(Uuid, String)>,
) -> Result<String, HandleError> {
    let mov = serde_json::from_str(&mov).unwrap();
    let user_id = user_id.into();
    let mut guard = db.lock().unwrap();
    let game = guard.get_mut(&game_id).ok_or(HandleError::NoGame)?;

    let mut succes_string = "Prima.".to_string();

    let current_player = game.state.current_player();
    if (game.p1 == user_id && current_player == Team::Red)
        || (game.p2 == Some(user_id) && current_player == Team::Blue)
    {
        if game.state.perform_mov(mov).is_err() {
            return Err(HandleError::InvalidMove);
        }
    } else {
        return Err(HandleError::OutOfTurn);
    }

    if game.state.winner().is_some() {
        if game.state.winner().unwrap() == Team::Red {
            succes_string = "Red won".to_string();
        } else {
            succes_string = "Blue won".to_string();
        }
    }
    Ok(succes_string)
}

async fn state(
    user_id: UserIdFromSession,
    Extension(db): Extension<Database>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<Game>, HandleError> {
    let mut guard = db.lock().unwrap();
    let game = guard.get_mut(&game_id).ok_or(HandleError::NoGame)?;
    let user_id = user_id.into();
    if game.p2.is_none() {
        if user_id == game.p1 {
            return Err(HandleError::NoSecondPlayer);
        } else {
            game.p2 = Some(user_id);
        }
    }
    Ok(Json(game.clone()))
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
            let mov: Action = serde_json::from_str(&message.into_text().unwrap()).unwrap();
            //   let game = db.lock().unwrap().get_mut(&game_id).unwrap();
            let current_player = db
                .lock()
                .unwrap()
                .get_mut(&game_id)
                .unwrap()
                .state
                .current_player();
            if (db.lock().unwrap().get_mut(&game_id).unwrap().p1 == user_id
                && current_player == Team::Red)
                || (db.lock().unwrap().get_mut(&game_id).unwrap().p2 == Some(user_id)
                    && current_player == Team::Blue)
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
                    == Team::Red
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let address = std::env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = std::env::var("PORT").unwrap_or("8000".to_owned());
    let host = format!("{address}:{port}");

    let db: Database = Arc::new(Mutex::new(HashMap::new()));
    let store = MemoryStore::new();
    let addr = host.parse().expect(&format!("invalid host: {host}"));
    let app = Router::new()
        .route("/", get(handler))
        .route("/card", get(card))
        .route("/create", get(create))
        .route("/connect/:game_id", get(connect))
        .route("/submit/:game_id/:mov", get(submit))
        .route("/state/:game_id", get(state))
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
