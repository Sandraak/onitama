use axum::extract::ws::WebSocket;
use axum::extract::{Path, WebSocketUpgrade};
use axum::response::Response;
use axum::Json;
use serde::Serialize;

use onitama::Onitama;

use crate::error::Error;
use crate::session::{Session, UserId};
use crate::store::{GameId, GameStore};

#[derive(Debug, Serialize)]
pub struct Game {
    players: Vec<UserId>,
    state: Onitama,
}

pub async fn create_game(session: Session, mut store: GameStore) -> Result<Json<GameId>, Error> {
    let game = Game {
        players: vec![session.id()],
        state: Default::default(),
    };
    let id = store.insert(&game).await?;

    Ok(Json(id))
}

pub async fn join_game(
    ws: WebSocketUpgrade,
    session: Session,
    mut store: GameStore,
    Path(id): Path<GameId>,
) -> Result<Response, Error> {
    let mut game = store.get(id).await?;
    if game.players.contains(&session.id()) {
        Ok(ws.on_upgrade(|socket| handle_socket(socket, game)))
    } else if game.players.len() < 2 {
        // TODO: Fix possible race condition
        game.players.push(session.id());
        store.update(id, &game).await?;
        Ok(ws.on_upgrade(|socket| handle_socket(socket, game)))
    } else {
        Err(Error::Full)
    }
}

async fn handle_socket(socket: WebSocket, game: Game) {
    todo!()
}
