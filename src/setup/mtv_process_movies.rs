use std::env;
use rusqlite::Connection;
use crate::setup::mtv_split;
use crate::setup::mtv_misc;

fn get_poster_addr(x: String) -> String {
    let no_ext_name_res = x.split(".");
    let mut no_ext_name_vec = vec![];

    for n in no_ext_name_res {
        no_ext_name_vec.push(n);
    }

    let new_jpg_name = no_ext_name_vec[0].to_owned() + ".jpg";
    let new_jpg_name_split = new_jpg_name.split("Movies");
    let mut new_jpg_name_split_vec = vec![];

    for jpg in new_jpg_name_split {
        new_jpg_name_split_vec.push(jpg);
    }
    let p1 = new_jpg_name_split_vec[0];
    let p2 = new_jpg_name_split_vec[1];
    let p2_split = p2.split("/");
    let mut p2_split_vec = vec![];
    for p in p2_split {
        p2_split_vec.push(p);
    }
    let poster_addr = p1.to_string() + &"Posters2/".to_string() + p2_split_vec[2];

    poster_addr
}

fn get_http_thumb_path(mname: String, year: String) -> String {
    let http = env::var("MTV_SERVER_ADDR").expect("MTV_SERVER_ADDR not set");
    let port = env::var("MTV_SERVER_PORT").expect("MTV_SERVER_PORT not set");
    let result = http + ":" + &port + "/thumbnails/" + &mname + " (" + &year + ")" + ".jpg";

    result
}

pub fn process_movies(x: String, count: u32) {
    let mov_name = mtv_split::split_movie_name(x.clone());
    let mov_year = mtv_split::split_movie_year(x.clone());
    let mov_poster_addr = get_poster_addr(x.clone());
    let mov_size = mtv_misc::get_file_size(&x);
    let mov_id = mtv_misc::create_md5(&x);
    let cat = parse_catagory(x.clone());
    let http_thumb_path = get_http_thumb_path(mov_name.clone(), mov_year.clone());
    
    let mojo = crate::setup::mtv_types::Movie {
        id: count,
        name: mov_name,
        year: mov_year,
        posteraddr: mov_poster_addr,
        size: mov_size.to_string(),
        path: x.clone(),
        idx: count.to_string(),
        movid: mov_id,
        catagory: cat,
        httpthumbpath: http_thumb_path,
    };
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute(
        "INSERT INTO movies (name, year, posteraddr, size, path, idx, movid, catagory, httpthumbpath) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        &[&mojo.name, &mojo.year, &mojo.posteraddr, &mojo.size, &mojo.path, &mojo.idx, &mojo.movid, &mojo.catagory, &mojo.httpthumbpath],
    )
    .expect("Unable to insert new tvshow info");
}

fn parse_catagory(x: String) -> String {
    if x.contains("Action") {
        return "Action".to_string();
    } else if x.contains("Arnold") {
        return "Arnold".to_string();
    } else if x.contains("BruceLee") {
        return "BruceLee".to_string();
    } else if x.contains("BruceWillis") {
        return "BruceWillis".to_string();
    } else if x.contains("Cartoons") {
        return "Cartoons".to_string();
    } else if x.contains("ChuckNorris") {
        return "ChuckNorris".to_string();
    } else if x.contains("Comedy") {
        return "Comedy".to_string();
    } else if x.contains("Drama") {
        return "Drama".to_string();
    } else if x.contains("Documentary") {
        return "Documentary".to_string();
    } else if x.contains("Fantasy") {
        return "Fantasy".to_string();
    } else if x.contains("Godzilla") {
        return "Godzilla".to_string();
    } else if x.contains("HarryPotter") {
        return "HarryPotter".to_string();
    } else if x.contains("IndianaJones") {
        return "IndianaJones".to_string();
    } else if x.contains("JamesBond") {
        return "JamesBond".to_string();
    } else if x.contains("JohnWayne") {
        return "JohnWayne".to_string();
    } else if x.contains("JohnWick") {
        return "JohnWick".to_string();
    } else if x.contains("JurassicPard") {
        return "JurassicPard".to_string();
    } else if x.contains("Kingsman") {
        return "Kingsman".to_string();
    } else if x.contains("MenInBlack") {
        return "MenInBlack".to_string();
    } else if x.contains("Misc") {
        return "Misc".to_string();
    } else if x.contains("NicolasCage") {
        return "NicolasCage".to_string();
    } else if x.contains("Pirates") {
        return "Pirates".to_string();
    } else if x.contains("Riddick") {
        return "Riddick".to_string();
    } else if x.contains("StarWars") {
        return "StarWars".to_string();
    } else if x.contains("StarTrek") {
        return "StarTrek".to_string();
    } else if x.contains("SuperHeroes") {
        return "SuperHeroes".to_string();
    } else if x.contains("SciFi") {
        return "SciFi".to_string();
    } else if x.contains("TomCruize") {
        return "TomCruize".to_string();
    } else if x.contains("Transformers") {
        return "Transformers".to_string();
    } else if x.contains("Tremors") {
        return "Tremors".to_string();
    } else if x.contains("TheRock") {
        return "TheRock".to_string();
    } else if x.contains("XMen") {
        return "XMen".to_string();
    } else {
        return "Unknown".to_string();
    };
}
