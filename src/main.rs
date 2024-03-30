use std::env;
use std::time::Instant;
use rusqlite::Connection;
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
    let start = Instant::now();

    // let _vars = envvars::set_env_vars();

    // let _dbfile = mtv_tables::create_db_file();

    let _tables = mtv_tables::create_tables();

    let movs = env::var("MTV_MOVIES_PATH").expect("$MTV_MOVIES_PATH is not set");
    let tvs = env::var("MTV_TV_PATH").expect("$MTV_TV_PATH is not set");

    let medialist = Vec::from([movs, tvs]);

    let mut thumbcount = String::new();
    let mut moviecount = String::new();
    let mut tvshowcount = String::new();
    let mut fsizevec: Vec<u64> = Vec::new();


    println!("medialist: {:?}", medialist.clone());

    for media in medialist {
        
            let media_vec_vec = mtv_walk_dirs::walk_movies_dir(media.clone());

            let fsize = media_vec_vec[3].clone();

            fsizevec.push(fsize[0].clone().parse().unwrap());



            let thumbnailz = media_vec_vec[2].clone();
            if thumbnailz.clone().len() > 0 {
                let mut count = 0;
                for thumb in thumbnailz {
                    count = count + 1;
                    let _process_movie_posters = mtv_image::process_movie_posters(
                        thumb.clone(),
                        count.clone(),
                    );
                };
                thumbcount = count.clone().to_string();
            }

            let moviez = media_vec_vec[0].clone();
            if moviez.clone().len() > 0 {
                let mut count = 0;
                for mov in moviez {
                    count = count + 1;
                    let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
                }
                moviecount = count.clone().to_string();
            }

            let tvshowz = media_vec_vec[1].clone();
            if tvshowz.clone().len() > 0 {
                let mut count = 0;
                for tv in tvshowz {
                    count = count + 1;
                    let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
                }
                tvshowcount = count.clone().to_string();
            }
        
    };

    let statz = mtv_types::Stats {
        id: 1,
        moviecount: moviecount,
        tvshowcount: tvshowcount,
        postercount: thumbcount,
        size: fsizevec.iter().sum::<u64>().to_string(),
    };
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute(
        "INSERT INTO stats (moviecount, tvshowcount, postercount, size) VALUES (?1, ?2, ?3, ?4)",
        &[&statz.moviecount, &statz.tvshowcount, &statz.postercount, &statz.size],

    )
    .expect("Unable to insert new tvshow info");

    let _write_file = mtv_utils::write_current_datetime_to_file(start.elapsed());

    Ok(())
}
