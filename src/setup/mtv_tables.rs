use rusqlite::{Connection, Result};
use std::env;
use std::path::Path;
use std::fs;

pub fn create_tables() {
    let _cmt = create_movies_table();
    let _ctvt = create_tvshows_table();
    let _cit = create_images_table();
    let _cst = create_stats_table();
}

pub fn create_movies_table() -> Result<()> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            year TEXT NOT NULL,
            posteraddr TEXT NOT NULL,
            size TEXT NOT NULL,
            path TEXT NOT NULL,
            idx TEXT NOT NULL,
            movid TEXT NOT NULL UNIQUE,
            catagory TEXT NOT NULL,
            httpthumbpath TEXT NOT NULL
         )",
        (),
    )?;

    Ok(())
}

pub fn create_tvshows_table() -> Result<()> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path.clone())?;
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

    Ok(())
}

pub fn create_images_table() -> Result<()> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path.clone())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            imgid TEXT NOT NULL UNIQUE,
            path TEXT NOT NULL,
            imgpath TEXT NOT NULL,
            size TEXT NOT NULL,
            name TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx INTEGER NOT NULL,
            httpthumbpath TEXT NOT NULL
         )",
        (),
    )?;

    Ok(())
}

pub fn create_stats_table() -> Result<()> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path.clone())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            moviecount TEXT NOT NULL,
            tvshowcount TEXT NOT NULL,
            postercount TEXT NOT NULL,
            size TEXT NOT NULL,
         )",
        (),
    )?;

    Ok(())
}

pub fn db_file_exists() -> bool {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let path = Path::new(&db_path);
    if path.exists() {
        return true;
    } else {
        return false;
    }
}

pub fn create_db_file() {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    fs::File::create(db_path.clone()).expect("Unable to create file");
}
