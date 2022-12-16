use axum::extract::{FromRef, FromRequestParts, Path};
use axum::http::request::Parts;
use axum_extra::extract::cookie::{Cookie, Key};
use axum_extra::extract::SignedCookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;

const SESSION_COOKIE: &str = "session";

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    id: UserId,
    name: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl Session {
    pub fn new(name: String) -> Self {
        Session {
            name,
            id: UserId(Uuid::new_v4()),
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar: SignedCookieJar<Key> = SignedCookieJar::from_request_parts(parts, state)
            .await
            .unwrap();
        match jar.get(SESSION_COOKIE) {
            None => Err(Error::Session),
            Some(cookie) => serde_json::from_str(cookie.value()).map_err(|_| Error::Session),
        }
    }
}

pub async fn create_session(
    Path(name): Path<String>,
    jar: SignedCookieJar,
) -> Result<SignedCookieJar, Error> {
    let session = Session::new(name);
    let cookie = Cookie::build(SESSION_COOKIE, serde_json::to_string(&session)?)
        .path("/")
        .finish();

    Ok(jar.add(cookie))
}
