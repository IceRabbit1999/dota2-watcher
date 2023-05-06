use std::collections::HashMap;
use crate::read_config;
use anyhow::{bail, Result};
use reqwest::get;
use serde::Serialize;
use serde_json::Value;
use tracing::info;

/// More info: https://wiki.teamfortress.com/wiki/WebAPI/GetMatchHistory
const MATCH_HISTORY: &str = "http://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/v1";
/// More info: https://wiki.teamfortress.com/wiki/WebAPI/GetMatchDetails
const MATCH_DETAILS: &str = "http://api.steampowered.com/IDOTA2Match_570/GetMatchDetails/v1";
const ALL_HEROES: &str = "http://api.steampowered.com/IEconDOTA2_570/GetHeroes/v1";

const ALL_ITEMS: &str = "";

pub struct Courier {
    pub client: reqwest::Client,
    /// Steam API key
    pub key: String,
}

/// Record some important fields in a match for outer interface
#[derive(Debug, Serialize)]
pub struct PlayerPerformance {
    pub account_id: String,
    pub match_id: String,
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
    pub game_mode: GameMode,
    pub hero_damage: u32,
    pub tower_damage: u32,
    pub hero_healing: u32,
    pub level: u8,
    pub dire_score: u32,
    pub radiant_score: u32,
}

impl PlayerPerformance {
    /// Construct PlayerPerformance from a match detail
    pub fn from_value(value: Value, account_id: &str) -> Result<Self> {
        let players = value.get("players").unwrap().as_array().unwrap();
        // game mode
        let game_mode = GameMode::from_u8(value.get("game_mode").unwrap().as_u64().unwrap() as u8)?;
        let dire_score = value.get("dire_score").unwrap().as_u64().unwrap() as u32;
        let radiant_score = value.get("radiant_score").unwrap().as_u64().unwrap() as u32;
        let radiant_win = value.get("radiant_win").unwrap().as_bool().unwrap();
        let match_id = value.get("match_id").unwrap().to_string();

        for p in players.iter() {
            if p.get("account_id").unwrap().as_u64().unwrap().to_string() == account_id {
                let hero_id = p.get("hero_id").unwrap().as_u64().unwrap() as u32;
                // 6 items + 1 neutral item
                let mut items = Vec::with_capacity(7);
                let kills = p.get("kills").unwrap().as_u64().unwrap() as u8;
                let deaths = p.get("deaths").unwrap().as_u64().unwrap() as u8;
                let gpm = p.get("gold_per_min").unwrap().as_u64().unwrap() as u32;
                let xpm = p.get("xp_per_min").unwrap().as_u64().unwrap() as u32;
                let assists = p.get("assists").unwrap().as_u64().unwrap() as u8;
                let last_hits = p.get("last_hits").unwrap().as_u64().unwrap() as u32;
                let denies = p.get("denies").unwrap().as_u64().unwrap() as u32;

                let hero_damage = p.get("hero_damage").unwrap().as_u64().unwrap() as u32;
                let tower_damage = p.get("tower_damage").unwrap().as_u64().unwrap() as u32;
                let hero_healing = p.get("hero_healing").unwrap().as_u64().unwrap() as u32;
                let level = p.get("level").unwrap().as_u64().unwrap() as u8;

                let radiant = match p.get("team_number").unwrap().as_u64().unwrap() {
                    0 => true,
                    1 => false,
                    _ => bail!("Unexpected team number when parse player performance")
                };
                let win = radiant && radiant_win || !radiant && !radiant_win;
                // item list
                for i in 0..items.capacity() {
                    if i == items.capacity() - 1 {
                        items.push(p.get("item_neutral").unwrap().as_u64().unwrap() as u32);
                        break;
                    }
                    let mut index = "item_".to_string();
                    index.push_str(&i.to_string());
                    items.push(p.get(index).unwrap().as_u64().unwrap() as u32)
                };

                return Ok(
                    Self {
                        account_id: account_id.to_string(),
                        match_id,
                        hero_id,
                        item_list: items,
                        kills,
                        deaths,
                        gpm,
                        xpm,
                        assists,
                        last_hits,
                        denies,
                        radiant,
                        win,
                        game_mode,
                        hero_damage,
                        tower_damage,
                        hero_healing,
                        level,
                        dire_score,
                        radiant_score
                    }
                );
            }
        }
        bail!("Failed to parse PlayerPerformance from value: {}", value);
    }
}

#[derive(Debug, Serialize)]
pub enum GameMode {
    AllPick = 1,
    RandomDraft = 3,
    RankedMatchmaking = 22,
    TurboMode = 23
}

impl GameMode {
    pub fn from_u8(mode: u8) -> Result<Self> {
        match mode {
            1 => Ok(GameMode::AllPick),
            3 => Ok(GameMode::RandomDraft),
            22 => Ok(GameMode::RankedMatchmaking),
            23 => Ok(GameMode::TurboMode),
            _ => bail!("Unsupported game mode found: {}", mode)
        }
    }
}

impl Courier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            key: read_config().api,
        }
    }

    pub async fn match_detail(&self, match_id: &str) -> Result<Value> {
        let res = self.client.get(MATCH_DETAILS.to_string())
            .query(&[("key", &self.key)])
            .query(&[("match_id", match_id)])
            .send().await?
            .json::<Value>().await?;
        let v = res.get("result").unwrap();
        // maybe there's a better way to make the borrow checker happy here
        Ok(v.clone())
    }

    /// Return a map with match_id and start_time based on the user account_id and matches_requested
    pub async fn match_history_with_account_id(&self, account_id: &str, matches_requested: u32) -> Result<HashMap<String, String>> {
        let res = self.client.get(MATCH_HISTORY.to_string())
            .query(&[("key", &self.key)])
            .query(&[("account_id", account_id)])
            .query(&[("matches_requested", matches_requested)])
            .send().await?
            .json::<Value>().await?;

        let match_list = res.get("result").unwrap().get("matches").unwrap().as_array().unwrap();
        let map = match_list.iter()
            .map(|v| (v.get("match_id").unwrap().to_string(), v.get("start_time").unwrap().to_string()))
            .collect::<HashMap<String, String>>();
        Ok(map)
    }


    /// Return a map with all heroes id and name
    pub async fn all_heroes(&self) -> Result<HashMap<u32, String>> {
        let res = self.client.get(ALL_HEROES.to_string())
            .query(&[("key", self.key.clone())])
            .send().await?
            .json::<Value>().await?;

        let hero_list = res.get("result").unwrap().get("heroes").unwrap().as_array().unwrap();
        let map = hero_list.iter()
            .map(|v| (v.get("id").unwrap().as_u64().unwrap() as u32, v.get("name").unwrap().to_string().replace('\"', "")))
            .collect::<HashMap<u32, String>>();
        Ok(map)
    }

    pub async fn all_items(&self) -> Result<HashMap<u32, String>> {
        let res = self.client.get(ALL_ITEMS.to_string())
            .query(&[("key", self.key.clone())])
            .send().await?
            .json::<Value>().await?;

        let item_list = res.get("result").unwrap().get("items").unwrap().as_array().unwrap();
        let map = item_list.iter()
            .map(|v| (v.get("id").unwrap().as_u64().unwrap() as u32, v.get("name").unwrap().to_string().replace('\"', "")))
            .collect::<HashMap<u32, String>>();
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{Courier, PlayerPerformance};
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
        println!("{res:?}");
        Ok(())
    }

    #[tokio::test]
    async fn transform() -> Result<()> {
        let client = Courier::new();
        let v = client.match_detail("7131970019").await?;
        println!("{v:?}");
        let result = PlayerPerformance::from_value(v, "417817047")?;
        println!("{result:?}");
        Ok(())
    }
}