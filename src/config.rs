use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: Log,
    pub api: String
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub path: String,
    pub prefix: String
}