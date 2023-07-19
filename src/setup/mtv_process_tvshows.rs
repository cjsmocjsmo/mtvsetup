use std::env;
use crate::setup::mtv_types;
use rusqlite::Connection;
use regex::Regex;

fn get_tv_catagory(x: String) -> String {
    let mut baz: String = String::new();
    if x.contains("S1") {
        let x_split = x.split("S1");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].to_string();
        println!("baz: {}", baz);
    } else if x.contains("S2") {
        let x_split = x.split("S2");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].trim().to_string();
        println!("baz: {}", baz);
    } else if x.contains("S3") {
        let x_split = x.split("S3");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].trim().to_string();
        println!("baz: {}", baz);
    } else if x.contains("S4") {
        let x_split = x.split("S4");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].trim().to_string();
        println!("baz: {}", baz);
    } else if x.contains("S5") {
        let x_split = x.split("S5");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].trim().to_string();
        println!("baz: {}", baz);
    } else if x.contains("S6") {
        let x_split = x.split("S6");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].trim().to_string();
        println!("baz: {}", baz);
    } else if x.contains("S7") {
        let x_split = x.split("S7");
        let mut x_split_vec = vec![];
        for xs in x_split {
            x_split_vec.push(xs);
        }
        let string_to_split = x_split_vec[1].to_string();
        let foo = Regex::new(r"S\d{2}").unwrap().split(&string_to_split).collect::<Vec<&str>>()[0].to_string();
        println!("foo: {}", foo);
        let mut foo_split_vec = Vec::new();
        let foo_split = foo.split("/");
        for f in foo_split {
            foo_split_vec.push(f);
        }

        baz = foo_split_vec[1].trim().to_string();
        println!("baz: {}", baz);
    } else {
        println!("Fuck")
    }

    
    baz
}

// fn get_tv_catagory(x: &String) -> String {
//     let name = crate::setup::mtv_split::split_movie_name(x.clone());
//     let n_split = name.split(" ");
//     let mut n_split_vec = vec![];
//     for n in n_split {
//         n_split_vec.push(n);
//     }
//     let idx = n_split_vec.len() - 2;
//     let mut newname_vec = vec![];
//     let foo = n_split_vec.drain(0..idx);
//     for f in foo {
//         newname_vec.push(f);
//     }
//     let bar = newname_vec.join(" ");

//     bar
// }

fn get_season(astring: String) -> String { 
    let my_captures: Vec<&str> = 
        Regex::new(r"S\d{2}")
            .unwrap().find_iter(&astring)
            .map(|x| x.as_str()).collect();
    let mut season = String::from("None");
    if my_captures.len() > 0 {
        season = my_captures[0].to_string();
    }
    let mut seavec = Vec::new();
    let parts = season.split("S");
    for p in parts {
        seavec.push(p);
    }

    seavec[1].to_string()
}

fn get_episode(astring: String) -> String { 
    let my_captures: Vec<&str> = 
        Regex::new(r"E\d{2}")
            .unwrap().find_iter(&astring)
            .map(|x| x.as_str()).collect();
    let mut episode = String::from("None");    
    if my_captures.len() > 0 {
        episode = my_captures[0].to_string();
    } 
    let mut epivec = Vec::new();
    let parts = episode.split("E");
    for p in parts {
        epivec.push(p);
    }
    
    epivec[1].to_string()
}

pub fn process_tvshows(tv: String, count: u32) {
    let catagory = get_tv_catagory(tv.clone());
    let season = get_season(tv.clone());
    let episode = get_episode(tv.clone());
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
