// use json::object;
// use std::env;
use crate::mtv_types;

fn get_tv_catagory(x: &String) -> String {
    let name = crate::mtv_split::split_movie_name(x.clone());
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
    let name = crate::mtv_split::split_movie_name(x.clone());
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



pub fn process_tvshows(tv: String, count: u32) -> String {
    let catagory = get_tv_catagory(&tv);
    let es = get_tv_episode_season(&tv);
    let season = es.0;
    let episode = es.1;
    let filesize = crate::mtv_misc::get_file_size(&tv);

    let fname = crate::mtv_split::split_filename(&tv.to_string());

    let tvshow = mtv_types::TVShow {
        size: filesize.to_string(),
        catagory: catagory,
        name: fname,
        season: season,
        episode: episode,
        path: tv,
        idx: count.to_string(),
    };

    // println!("{:#?}", tvshow);

    // let tvshows_obj = object! {
    //     size: file_size,
    //     catagory: catagory,
    //     name: fname,
    //     season: season,
    //     episode: episode,
    //     path: tv
    // };

    // let tvsows_info = json::stringify(tvshows_obj.dump());

    // let mtv_tvshows_metadata_path =
    //     env::var("MTV_TVSHOWS_METADATA_PATH").expect("$MTV_TVSHOWS_METADATA_PATH is not set");

    // let a = format!("{}/", mtv_tvshows_metadata_path.as_str());
    // let b = format!("TVShows_{}_Meta.json", count.to_string());
    // let outpath = a + &b;

    // std::fs::write(outpath, tvsows_info).unwrap();

    "fuck".to_string()
}
