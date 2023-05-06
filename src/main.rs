mod config;
mod client;
mod service;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::get;
use anyhow::Result;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Local;
use tracing::{info, Level};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::client::{Courier, PlayerPerformance};
use crate::config::Config;
use crate::service::{latest_match, subscribe_player};


#[tokio::main]
async fn main() -> Result<()>{
    // init log
    let config = read_config();

    let file_appender = tracing_appender::rolling::daily(&config.log.path, &config.log.prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(io::stdout)
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();

    let state = AppState::new().await?;

    let app = Router::new()
        .route("/match/latest", get(latest_match))
        .route("/subscribe", get(subscribe_player))
        .with_state(state);


    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}

/// Wrapper anyhow:Error so we can impl necessary trait
pub struct AppError(anyhow::Error);
/// To make handlers which return Result<AnyType, anyhow::Error> compile because of the trait bound
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

/// To enable `?` usage
impl<E> From<E> for AppError where E: Into<anyhow::Error>{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

pub fn read_config() -> Config {
    let mut file = File::open("config/app.toml").expect("Can not open config/app.toml");
    let mut config_str = String::new();
    file.read_to_string(&mut config_str).expect("Reading config/app.toml failed");
    toml::from_str(&config_str).expect("config from str failed")
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Courier>,
    pub performance_cache: Arc<Mutex<HashMap<(String, String), PlayerPerformance>>>,
    pub subscribe_cache: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub hero_map: HashMap<u32, String>,
    pub item_map: HashMap<u32, String>
}

impl AppState {
    async fn new() -> Result<Self> {
        let client = Courier::new();
        // init hero and item map
        let hero_map = client.all_heroes().await?;
        let item_map = client.all_items().await?;
        Ok(Self {
            client: Arc::new(client),
            performance_cache: Arc::new(Mutex::new(HashMap::new())),
            subscribe_cache: Arc::new(Mutex::new(HashMap::new())),
            hero_map,
            item_map
        })
    }
}
