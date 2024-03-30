use dotenv::dotenv;
use rayon::prelude::*;
use rusqlite::Connection;
use std::env;
use std::time::Instant;

pub mod mtv_image;
mod mtv_process_movies;
mod mtv_process_tvshows;
pub mod mtv_tables;
pub mod mtv_types;
pub mod mtv_utils;
mod mtv_walk_dirs;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let start = Instant::now();
    let _tables = mtv_tables::create_tables();

    let poster_path = env::var("MTV_POSTER_PATH").expect("$MTV_POSTER_PATH is not set");
    let poster_list = mtv_walk_dirs::walk_posters_dir(poster_path.clone());
    let poster_count = poster_list.len();
    if !mtv_image::thumbnail_dir_exists() {
        mtv_image::create_thumbnail_dir();
    }
    poster_list
        .into_par_iter()
        .enumerate()
        .for_each(|(count, poster)| {
            mtv_image::process_posters(poster, (count + 1).try_into().unwrap());
        });

    let mov_path = env::var("MTV_MOVIES_PATH").expect("$MTV_MOVIES_PATH is not set");
    let mov_list = mtv_walk_dirs::walk_movies_dir(mov_path.clone());
    let mov_count = mov_list.len();
    mov_list
        .into_par_iter()
        .enumerate()
        .for_each(|(count, mov)| {
            mtv_process_movies::process_movies(mov, (count + 1).try_into().unwrap());
        });

    let tv_path = env::var("MTV_TV_PATH").expect("$MTV_TV_PATH is not set");
    let tv_list = mtv_walk_dirs::walk_tvshows_dir(tv_path.clone());
    let tv_count = tv_list.len();
    tv_list.into_par_iter().enumerate().for_each(|(count, tv)| {
        mtv_process_tvshows::process_tvshows(tv, (count + 1).try_into().unwrap());
    });

    println!("Starting statz");
    let statz = mtv_types::Stats {
        id: 1,
        moviecount: mov_count.to_string(),
        tvshowcount: tv_count.to_string(),
        postercount: poster_count.to_string(),
        size: "0".to_string(),
    };
    println!("statz: {:?}", statz);
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute(
        "INSERT INTO stats (moviecount, tvshowcount, postercount, size) VALUES (?1, ?2, ?3, ?4)",
        &[
            &statz.moviecount,
            &statz.tvshowcount,
            &statz.postercount,
            &statz.size,
        ],
    )
    .expect("Unable to insert new tvshow info");

    let _write_file = mtv_utils::write_current_datetime_to_file(start.elapsed());

    Ok(())
}
