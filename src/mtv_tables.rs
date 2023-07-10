use rusqlite::{Connection, Result};
use std::env;

pub fn create_tables() -> Result<()> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            year TEXT NOT NULL,
            poster_addr TEXT NOT NULL,
            size TEXT NOT NULL,
            exists BOOL NOT NULL,
            path TEXT NOT NULL,
            index TEXT NOT NULL,
            movid TEXT NOT NULL UNIQUE
         )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tvshows (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tvid TEXT NOT NULL UNIQUE,
            size TEXT NOT NULL,
            catagory TEXT NOT NULL,
            name TEXT NOT NULL,
            season TEXT NOT NULL,
            episode TEXT NOT NULL,
            path TEXT NOT NULL,
            idx TEXT NOT NULL
         )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS admin (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            imgid TEXT NOT NULL UNIQUE,
            path TEXT NOT NULL,
            imgpath TEXT NOT NULL,
            size TEXT NOT NULL,
            name TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx INTEGER NOT NULL
         )",
        (),
    )?;

    Ok(())
}
