use std::collections::HashMap;
use crate::read_config;
use anyhow::Result;

/// More info: https://wiki.teamfortress.com/wiki/WebAPI/GetMatchHistory
const MATCH_HISTORY: &str = "http://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/v1";
/// More info: https://wiki.teamfortress.com/wiki/WebAPI/GetMatchDetails
const MATCH_DETAILS: &str = "http://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/v1";
const ALL_HEROES: &str = "http://api.steampowered.com/IEconDOTA2_570/GetHeroes/v1";

pub struct Courier {
    pub client: reqwest::Client,
    /// Steam API key
    pub key: String,
}
/// Record some important fields in a match for outer interface
pub struct PlayerPerformance {
    pub account_id: String,
    pub hero_id: u32,
    /// Items list, from left to right, top to bottom
    pub item_list: Vec<u32>,
    pub kills: u8,
    pub deaths: u8,
    pub assists: u8,
    pub last_hits: u32,
    pub denies: u32,
    pub gpm: u32,
    pub xpm: u32,
    /// True if player team win the game
    pub win: bool,
    /// True if player is in radiant team
    pub radiant: bool,
    pub game_mode: u8,
    pub hero_damage: u32,
    pub tower_damage: u32,
    pub hero_healing: u32,
    pub level: u8,
    pub dire_score: u32,
    pub radiant_score: u32,
}

impl Courier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            key: read_config().api,
        }
    }

    pub async fn match_detail(&self, match_id: &str) -> Result<()>{
        let res = self.client.get(MATCH_DETAILS.to_string())
            .query(&[("key", &self.key)])
            .query(&[("match_id", match_id)])
            .send().await?
            .json::<serde_json::Value>().await?;
        println!("{:?}", res);
        Ok(())
    }

    /// Return a map with match_id and start_time based on the user account_id and matches_requested
    pub async fn match_history_with_account_id(&self, account_id: &str, matches_requested: u32) -> Result<HashMap<String, String>>{
        let res = self.client.get(MATCH_HISTORY.to_string())
            .query(&[("key", &self.key)])
            .query(&[("account_id", account_id)])
            .query(&[("matches_requested", matches_requested)])
            .send().await?
            .json::<serde_json::Value>().await?;

        let match_list = res.get("result").unwrap().get("matches").unwrap().as_array().unwrap();
        let map = match_list.iter()
            .map(|v| (v.get("match_id").unwrap().to_string(), v.get("start_time").unwrap().to_string()))
            .collect::<HashMap<String, String>>();
        println!("{:?}", res);
        Ok(map)
    }


    /// Return a map with all heroes id and name
    pub async fn all_heroes(&self) -> Result<HashMap<u64, String>>{
        let res = self.client.get(ALL_HEROES.to_string())
            .query(&[("key", self.key.clone())])
            .send().await?
            .json::<serde_json::Value>().await?;

        let hero_list = res.get("result").unwrap().get("heroes").unwrap().as_array().unwrap();
        let map = hero_list.iter()
            .map(|v| (v.get("id").unwrap().as_u64().unwrap(), v.get("name").unwrap().to_string().replace('\"', "")))
            .collect::<HashMap<u64, String>>();
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Courier;
    use anyhow::Result;
    use serde_json::json;

    #[tokio::test]
    async fn hero() -> Result<()> {
        let client = Courier::new();
        let res = client.all_heroes().await?;
        println!("{:?}", res);
        Ok(())
    }

    #[tokio::test]
    async fn history() -> Result<()> {
        let client = Courier::new();
        let res = client.match_history_with_account_id("417817047", 1).await?;
        println!("{:?}", res);
        Ok(())
    }

    #[tokio::test]
    async fn detail() -> Result<()> {
        let client = Courier::new();
        let res = client.match_detail("7124687646").await?;

        Ok(())
    }
}