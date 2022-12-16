use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::extract::FromRef;
use axum::middleware::from_extractor_with_state;
use axum::routing::get;
use axum::{Router, Server};
use axum_extra::extract::cookie::Key;
use redis::Client;
use tracing::info;
use tracing_subscriber::fmt;

use crate::session::Session;

mod error;
mod game;
mod session;
mod store;

const ENV_HOST: &str = "HOST";
const ENV_PORT: &str = "PORT";
const ENV_REDIS: &str = "REDIS_URL";

const DEFAULT_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
const DEFAULT_PORT: u16 = 8000;

async fn index(session: Session) -> String {
    format!("Hello, {}!", session.name())
}

#[derive(Clone)]
struct AppState {
    client: Client,
    key: Key,
}

impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.client.clone()
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() {
    fmt::init();

    let host = env::var(ENV_HOST)
        .map(|host| host.parse().expect("invalid host"))
        .unwrap_or(DEFAULT_HOST);
    let port = env::var(ENV_PORT)
        .map(|port| port.parse().expect("invalid port"))
        .unwrap_or(DEFAULT_PORT);
    let addr = SocketAddr::new(host, port);

    let redis = env::var(ENV_REDIS).expect(ENV_REDIS);

    let state = AppState {
        client: Client::open(redis).expect("invalid Redis URL"),
        key: Key::generate(),
    };

    let app = Router::new()
        .route("/", get(index))
        .route_layer(from_extractor_with_state::<Session, _>(state.clone()))
        .route("/create_session/:name", get(session::create_session))
        .with_state(state);

    info!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server closed");
}
