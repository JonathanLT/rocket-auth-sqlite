use rusqlite::{Connection, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn init_db() -> Result<()> {
    let conn = Connection::open("auth.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn create_user(username: &str, password: &str) -> Result<()> {
    let conn = Connection::open("auth.db")?;
    let password_hash = hash(password, DEFAULT_COST).unwrap();

    conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        [username, &password_hash],
    )?;

    Ok(())
}

pub fn validate_user(username: &str, password: &str) -> Result<bool> {
    let conn = Connection::open("auth.db")?;
    let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE username = ?1")?;
    let mut rows = stmt.query([username])?;

    if let Some(row) = rows.next()? {
        let password_hash: String = row.get(0)?;
        Ok(verify(password, &password_hash).unwrap_or(false))
    } else {
        Ok(false)
    }
}