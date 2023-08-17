use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use redis::aio::Connection;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;
use crate::game::Game;

pub struct GameStore {
    connection: Connection,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameId(Uuid);

impl GameStore {
    pub async fn insert(&mut self, game: &Game) -> Result<GameId, Error> {
        let id = GameId(Uuid::new_v4());
        self.connection
            .set(id.0.as_bytes(), serde_json::to_string(game)?)
            .await
            .map_err(|_| Error::Store)?;

        Ok(id)
    }

    pub async fn update(&mut self, id: GameId, game: &Game) -> Result<(), Error> {
        self.connection
            .set(id.0.as_bytes(), serde_json::to_string(game)?)
            .await
            .map_err(|_| Error::Store)
    }

    pub async fn get(&mut self, id: GameId) -> Result<Game, Error> {
        self.connection
            .get::<_, String>(id.0.as_bytes())
            .await
            .ok()
            .and_then(|x| serde_json::from_str(&x).ok())
            .ok_or(Error::DoesNotExist)
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for GameStore
where
    S: Sync,
    Client: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let client = Client::from_ref(state);
        let connection = client
            .get_async_connection()
            .await
            .map_err(|_| Error::Store)?;
        let database = GameStore { connection };

        Ok(database)
    }
}
