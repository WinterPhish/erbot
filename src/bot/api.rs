use dotenv::var;

use std::sync::OnceLock;

const WEBSITE: &str = "https://open-api.bser.io";
fn password() -> &'static str {
    static API_KEY: OnceLock<String> = OnceLock::new();

    API_KEY.get_or_init(|| var("ER_API_TOKEN").unwrap())
}

pub async fn get_userid(username: String) -> Result<serde_json::Value, ureq::Error> {
    let link: String = format!("{}{}{}", WEBSITE, "/v1/user/nickname?query=", username);
    let res: serde_json::Value = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}

pub async fn get_user_stats(
    userid: String,
    seasonid: String,
) -> Result<serde_json::Value, ureq::Error> {
    let link: String = format!(
        "{}{}{}{}{}",
        "WEBSITE", "/v1/user/stats/", userid, "/", seasonid
    );
    let res: serde_json::Value = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}

pub async fn get_user_games(userid: String) -> Result<serde_json::Value, ureq::Error> {
    let link: String = format!("{}{}{}", "WEBSITE", "/v1/user/games/", userid,);
    let res: serde_json::Value = ureq::get(&link)
        .set("x-api-key", password())
        .call()?
        .into_json()?;
    Ok(res)
}
