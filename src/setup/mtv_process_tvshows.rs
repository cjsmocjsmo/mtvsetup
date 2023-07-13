use std::env;
use crate::setup::mtv_types;
use rusqlite::Connection;

fn get_tv_catagory(x: &String) -> String {
    let name = crate::setup::mtv_split::split_movie_name(x.clone());
    let n_split = name.split(" ");
    let mut n_split_vec = vec![];

    for n in n_split {
        n_split_vec.push(n);
    }

    let idx = n_split_vec.len() - 2;

    let mut newname_vec = vec![];

    let foo = n_split_vec.drain(0..idx);

    for f in foo {
        newname_vec.push(f);
    }

    let bar = newname_vec.join(" ");

    bar
}

fn get_tv_episode_season(x: &String) -> (String, String) {
    let name = crate::setup::mtv_split::split_movie_name(x.clone());
    let n_split = name.split(" ");
    let mut n_split_vec = vec![];

    for n in n_split {
        n_split_vec.push(n);
    }
    let idx = n_split_vec.len() - 2;

    let parts: Vec<char> = n_split_vec[idx].chars().collect();

    let season = parts[1].to_string() + &parts[2].to_string();
    let episode = parts[4].to_string() + &parts[5].to_string();

    let results = (season, episode);

    results
}

pub fn process_tvshows(tv: String, count: u32) {
    let catagory = get_tv_catagory(&tv);
    let es = get_tv_episode_season(&tv);
    let season = es.0;
    let episode = es.1;
    let filesize = crate::setup::mtv_misc::get_file_size(&tv);

    let fname = crate::setup::mtv_split::split_filename(&tv.to_string());

    let tvshow = mtv_types::TVShow {
        id: count,
        tvid: crate::setup::mtv_misc::create_md5(&tv),
        size: filesize.to_string(),
        catagory: catagory,
        name: fname,
        season: season,
        episode: episode,
        path: tv,
        idx: count.to_string(),
    };
    println!("{:#?}", tvshow);
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute(
        "INSERT INTO tvshows (tvid, size, catagory, name, season, episode, path, idx) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[&tvshow.tvid, &tvshow.size, &tvshow.catagory, &tvshow.name, &tvshow.season, &tvshow.episode, &tvshow.path, &tvshow.idx],
        
    )
    .expect("Unable to insert new tvshow info");

    
}