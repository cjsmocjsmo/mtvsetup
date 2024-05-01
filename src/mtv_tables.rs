use rusqlite::{Connection, Result};
use std::env;
use std::path::Path;
// use std::fs;

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
            Name TEXT NOT NULL,
            Year TEXT NOT NULL,
            PosterAddr TEXT NOT NULL,
            Size TEXT NOT NULL,
            Path TEXT NOT NULL,
            Idx TEXT NOT NULL,
            MovId TEXT NOT NULL UNIQUE,
            Catagory TEXT NOT NULL,
            HttpThumbPath TEXT NOT NULL
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
            TvId TEXT NOT NULL UNIQUE,
            Size TEXT NOT NULL,
            Catagory TEXT NOT NULL,
            Name TEXT NOT NULL,
            Season TEXT NOT NULL,
            Episode TEXT NOT NULL,
            Path TEXT NOT NULL,
            Idx TEXT NOT NULL
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
            ImgId TEXT NOT NULL UNIQUE,
            Path TEXT NOT NULL,
            ImgPath TEXT NOT NULL,
            Size TEXT NOT NULL,
            Name TEXT NOT NULL,
            ThumbPath TEXT NOT NULL,
            Idx INTEGER NOT NULL,
            HttpThumbPath TEXT NOT NULL
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
            MovieCount TEXT NOT NULL,
            TvShowCount TEXT NOT NULL,
            PosterCount TEXT NOT NULL,
            Size TEXT NOT NULL
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

// pub fn create_db_file() {
//     let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
//     fs::File::create(db_path.clone()).expect("Unable to create file");
// }
