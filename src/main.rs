mod config;
mod client;

use std::fs::File;
use std::io;
use std::io::Read;
use axum::Router;
use axum::routing::get;
use anyhow::Result;
use chrono::Local;
use tracing::{info, Level};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::config::Config;


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
        .with_max_level(Level::DEBUG)
        .with_writer(io::stdout)
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();


    let app = Router::new().route("/", get(handler));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

async fn handler() -> &'static str {
    "Hello"
}

pub fn read_config() -> Config {
    let mut file = File::open("config/app.toml").expect("Can not open config/app.toml");
    let mut config_str = String::new();
    file.read_to_string(&mut config_str).expect("Reading config/app.toml failed");
    toml::from_str(&config_str).expect("config from str failed")
}


