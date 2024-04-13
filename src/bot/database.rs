use rusqlite::{Connection, Result};

pub const DB_NAME: &str = "accounts.sqlite";

#[derive(Debug)]
pub struct AccountLink {
    pub er_id: String,
    pub discord_id: String,
}

pub fn init_database() -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    conn.execute(
        "CREATE TABLE LinkedAccounts (
            discord_id  TEXT UNIQUE NOT NULL,
            er_id       TEXT UNIQUE NOT NULL
        )",
        (),
    )?;
    Ok(())
}

pub async fn add_data(discord_id: String, er_id: String) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DB_NAME).unwrap();

    match conn.execute(
        "INSERT INTO LinkedAccounts (discord_id, er_id) VALUES (?1, ?2)",
        (discord_id, er_id),
    ) {
        Ok(updated) => {
            println!("{} rows were updated", updated);
            Ok(())
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(err)
        }
    }
}

pub async fn query_discord_id(discord_id: String) -> Result<AccountLink> {
    let conn = Connection::open(DB_NAME).unwrap();

    let mut q = conn
        .prepare("SELECT discord_id, er_id FROM LinkedAccounts WHERE discord_id =:discord_id;")?;
    let mut acc_iter = q.query_map(&[(":discord_id", discord_id.to_string().as_str())], |row| {
        Ok(AccountLink {
            discord_id: row.get(0)?,
            er_id: row.get(1)?,
        })
    })?;

    let account = acc_iter.next().unwrap()?;

    Ok(account)
}

pub async fn query_er_id(er_id: String) -> Result<AccountLink> {
    let conn = Connection::open(DB_NAME).unwrap();
    let mut q =
        conn.prepare("SELECT discord_id, er_id FROM LinkedAccounts WHERE er_id =:er_id;")?;
    let mut acc_iter = q.query_map(&[(":er_id", er_id.to_string().as_str())], |row| {
        Ok(AccountLink {
            discord_id: row.get(0)?,
            er_id: row.get(1)?,
        })
    })?;

    let account = acc_iter.next().unwrap()?;

    Ok(account)
}
