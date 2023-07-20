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

        baz = foo_split_vec[1].to_string();
    } else {
        println!("Fuck")
    }

    
    baz.trim().to_string()
}

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

fn get_tv_name(x: &String) -> String {
    if x.contains("AlteredCarbon") {
        return String::from("Altered Carbon");
    } else if x.contains("CowboyBebop") {
        return String::from("Cowboy Bebop");
    } else if x.contains("ForAllManKind") {
        return String::from("For All ManKind");
    } else if x.contains("Foundation") {
        return String::from("Foundation");
    } else if x.contains("FuuBar") {
        return String::from("FuuBar");
    } else if x.contains("Halo") {
        return String::from("Halo");
    } else if x.contains("HFord1923") {
        return String::from("1923");
    } else if x.contains("HouseOfTheDragon") {
        return String::from("House Of The Dragon");
    } else if x.contains("LostInSpace") {
        return String::from("Lost In Space");
    } else if x.contains("MastersOfTheUniverse") {
        return String::from("Masters Of The Universe");
    } else if x.contains("NightSky") {
        return String::from("Night Sky");
    } else if x.contains("Orville") {
        return String::from("Orville");
    } else if x.contains("RaisedByWolves") {
        return String::from("Raised By Wolves");
    } else if x.contains("Silo") {
        return String::from("Silo");
    } else if x.contains("StarTrek/Discovery") {
        return String::from("Discovery");
    } else if x.contains("StarTrek/Picard") {
        return String::from("Picard");
    } else if x.contains("StarTrek/StrangeNewWorlds") {
        return String::from("Strange New Worlds");
    } else if x.contains("StarTrek/LowerDecks") {
        return String::from("Lower Decks");
    } else if x.contains("StarTrek/Prodigy") {
        return String::from("Prodigy");
    } else if x.contains("StarTrek/TNG") {
        return String::from("The Next Generation");
    } else if x.contains("StarTrek/STTV") {
        return String::from("Star Trek");
    } else if x.contains("StarTrek/Voyager") {
        return String::from("Voyager");
    } else if x.contains("StarTrek/ENT") {
        return String::from("Enterprise");
    } else if x.contains("Andor") {
        return String::from("Andor");
    } else if x.contains("BookOfBobaFett") {
        return String::from("Boba Fett");
    } else if x.contains("Mandalorian") {
        return String::from("Mandalorian");
    } else if x.contains("ObiWanKenobi") {
        return String::from("Obi Wan");
    } else if x.contains("TalesOfTheJedi") {
        return String::from("Tales Of The Jedi");
    } else if x.contains("TheBadBatch") {
        return String::from("Bad Batch");
    } else if x.contains("Visions") {
        return String::from("Visions");
    } else if x.contains("TheLastOfUs") {
        return String::from("The Last Of Us");
    } else if x.contains("TheLordOfTheRingsTheRingsOfPower") {
        return String::from("Rings Of Power");
    } else if x.contains("WheelOfTime") {
        return String::from("Wheel Of Time");
    } else if x.contains("FalconWinterSoldier") {
        return String::from("Falcon Winter Soldier");
    } else if x.contains("Hawkeye") {
        return String::from("Hawkeye");
    } else if x.contains("IAmGroot") {
        return String::from("I Am Groot");
    } else if x.contains("Loki") {
        return String::from("Loki");
    } else if x.contains("MoonKnight") {
        return String::from("Moon Knight");
    } else if x.contains("MSMarvel") {
        return String::from("Ms Marvel");
    } else if x.contains("SecretInvasion") {
        return String::from("Secret Invasion");
    } else if x.contains("SheHulk") {
        return String::from("She Hulk");
    } else if x.contains("WandaVision") {
        return String::from("WandaVision");
    } else {
        return String::from("Fuck");
    }
    
}

pub fn process_tvshows(tv: String, count: u32) {
    let catagory = get_tv_catagory(tv.clone());
    let season = get_season(tv.clone());
    let episode = get_episode(tv.clone());
    let filesize = crate::setup::mtv_utils::get_file_size(&tv);
    let fname = get_tv_name(&tv);
    let tvshow = mtv_types::TVShow {
        id: count,
        tvid: crate::setup::mtv_utils::create_md5(&tv),
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
