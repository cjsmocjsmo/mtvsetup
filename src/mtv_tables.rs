use rusqlite::{Connection, Result};
use std::env;

pub fn create_tables() -> Result<()> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path)?;

    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
            id INTEGER PRIMARY KEY,
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
        "CREATE TABLE IF NOT EXISTS reviews (
             id INTEGER PRIMARY KEY,
             acctid TEXT NOT NULL,
             revid TEXT NOT NULL UNIQUE,
             name TEXT NOT NULL,
             email TEXT NOT NULL,
             stars TEXT NOT NULL,
             review TEXT NOT NULL,
             reject TEXT NOT NULL,
             accept TEXT NOT NULL,
             jailed TEXT NOT NULL
         )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS admin (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE,
             password TEXT NOT NULL UNIQUE,
             email TEXT NOT NULL UNIQUE,
             token TEXT NOT NULL UNIQUE
         )",
        (),
    )?;

    Ok(())
}
