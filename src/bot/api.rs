#![allow(non_snake_case)]
use dotenv::var;
use serde::{Deserialize, Serialize};
use std::{f32, sync::OnceLock};
const WEBSITE: &str = "https://open-api.bser.io";
fn password() -> &'static str {
    static API_KEY: OnceLock<String> = OnceLock::new();

    API_KEY.get_or_init(|| var("ER_API_TOKEN").unwrap())
}

// THE STRUCTURES IN QUESTION
#[derive(Serialize, Deserialize)]
pub struct Nickname {
    pub code: u8,
    pub message: String,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub userNum: u32,
    pub nickname: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserStats {
    pub code: u8,
    pub message: String,
    pub userStats: Vec<Stats>,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub seasonId: u8,
    pub userNum: u32,
    pub matchingMode: u8,
    pub nickname: String,
    pub rank: u32,
    pub rankSize: u32,
    pub totalGames: u32,
    pub totalWins: u32,
    pub rankPercent: f32,
    pub averageRank: f32,
    pub averageKills: f32,
    pub averageAssistants: f32,
    pub averageHunts: f32,
    pub top1: f32,
    pub top2: f32,
    pub top3: f32,
    pub top5: f32,
    pub top7: f32,
    pub characterStats: Vec<CharacterStats>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterStats {
    pub characterCode: u8,
    pub totalGames: u8,
    pub usages: u8,
    pub maxKillings: u8,
    pub top3: u8,
    pub wins: u8,
    pub top3Rate: f32,
    pub averageRank: u8,
}

pub async fn get_userid(username: String) -> Result<Nickname, ureq::Error> {
    let link: String = format!("{}{}{}", WEBSITE, "/v1/user/nickname?query=", username);
    let res: Nickname = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}

pub async fn get_user_stats(userid: String, seasonid: String) -> Result<UserStats, ureq::Error> {
    let link: String = format!(
        "{}{}{}{}{}",
        WEBSITE, "/v1/user/stats/", userid, "/", seasonid
    );
    let res: UserStats = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}

pub async fn get_user_games(userid: String) -> Result<serde_json::Value, ureq::Error> {
    let link: String = format!("{}{}{}", WEBSITE, "/v1/user/games/", userid,);
    let res: serde_json::Value = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}

pub async fn get_game(gameid: String) -> Result<serde_json::Value, ureq::Error> {
    let link: String = format!("{}{}{}", WEBSITE, "v1/games/", gameid);
    let res: serde_json::Value = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}
