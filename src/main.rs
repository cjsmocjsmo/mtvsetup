// use rusqlite::Connection;
use std::env;
// use std::time::Instant;
// mod envvars;
use dotenv::dotenv;
pub mod mtv_image;
mod mtv_process_movies;
mod mtv_process_tvshows;
pub mod mtv_tables;
pub mod mtv_types;
pub mod mtv_utils;
mod mtv_walk_dirs;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // let start = Instant::now();

    let poster_path = env::var("MTV_POSTER_PATH").expect("$MTV_POSTER_PATH is not set");

    let poster_list = mtv_walk_dirs::walk_posters_dir(poster_path.clone());

    let mut count = 0;
    for poster in poster_list {
        count = count + 1;
        let _process_movie_posters =
            mtv_image::process_posters(poster.clone(), count.clone());
    }

    // let _tables = mtv_tables::create_tables();

    // let movs = env::var("MTV_MOVIES_PATH").expect("$MTV_MOVIES_PATH is not set");
    // let tvs = env::var("MTV_TV_PATH").expect("$MTV_TV_PATH is not set");

    // let medialist = Vec::from([movs, tvs]);

    // let mut mov_vec = Vec::new();
    // let mut tv_vec = Vec::new();

    // let mut fsize_vec = Vec::new();

    // for media in medialist {
    //     let media_vec_vec = mtv_walk_dirs::walk_movies_dir(media.clone());
    //     mov_vec = media_vec_vec[0].clone();
    //     tv_vec = media_vec_vec[1].clone();
    //     post_vec = media_vec_vec[2].clone();
    //     fsize_vec = media_vec_vec[3].clone();
    // }

    // let thumbcount = post_vec.len().to_string();
    // let moviecount = mov_vec.len().to_string();
    // let tvshowcount = tv_vec.len().to_string();

    // println!("fsizes: {:?}", fsize_vec.clone());
    // println!("Thumbcount {:?}", thumbcount.clone());
    // println!("Moviecount {:?}", moviecount.clone());
    // println!("Tvshowcount {:?}", tvshowcount.clone());

    //         let moviez = media_vec_vec[0].clone();
    //         if moviez.clone().len() > 0 {
    //             let mut count = 0;
    //             for mov in moviez {
    //                 count = count + 1;
    //                 let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
    //             }
    //             moviecount = count.clone().to_string();
    //         }

    //         let tvshowz = media_vec_vec[1].clone();
    //         if tvshowz.clone().len() > 0 {
    //             let mut count = 0;
    //             for tv in tvshowz {
    //                 count = count + 1;
    //                 let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
    //             }
    //             tvshowcount = count.clone().to_string();
    //         }

    // };

    // let statz = mtv_types::Stats {
    //     id: 1,
    //     moviecount: moviecount,
    //     tvshowcount: tvshowcount,
    //     postercount: thumbcount,
    //     size: fsizevec.iter().sum::<u64>().to_string(),
    // };
    // let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    // let conn = Connection::open(db_path).expect("unable to open db file");
    // conn.execute(
    //     "INSERT INTO stats (moviecount, tvshowcount, postercount, size) VALUES (?1, ?2, ?3, ?4)",
    //     &[&statz.moviecount, &statz.tvshowcount, &statz.postercount, &statz.size],

    // )
    // .expect("Unable to insert new tvshow info");

    // let _write_file = mtv_utils::write_current_datetime_to_file(start.elapsed());

    Ok(())
}
