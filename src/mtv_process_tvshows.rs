use std::env;
use crate::mtv_types;
use rusqlite::Connection;
use regex::Regex;

fn get_season(astring: &String) -> String {
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
        println!("astring: {:?}", astring);
        println!("p: {:?}", p);
        seavec.push(p);
    }

    seavec[1].to_string()
}

fn get_episode(astring: &String) -> String {
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

fn get_tv_catagory_name(x: &String) -> (String, String) {
    if x.contains("AlteredCarbon") {
        return (String::from("AlteredCarbon"), String::from("Altered Carbon"));
    } else if x.contains("CowboyBebop") {
        return (String::from("CowboyBebop"), String::from("Cowboy Bebop"));
    } else if x.contains("ForAllManKind") {
        return (String::from("ForAllManKind"), String::from("For All ManKind"));
    } else if x.contains("Foundation") {
        return (String::from("Foundation"), String::from("Foundation"));
    } else if x.contains("FuuBar") {
        return (String::from("FuuBar"), String::from("FuuBar"));
    } else if x.contains("Halo") {
        return (String::from("Halo"), String::from("Halo"));
    } else if x.contains("HFord1923") {
        return (String::from("HFord1923"), String::from("Harrison Ford 1923"));
    } else if x.contains("HouseOfTheDragon") {
        return (String::from("HouseOfTheDragon"), String::from("House Of The Dragon"));
    } else if x.contains("LostInSpace") {
        return (String::from("LostInSpace"), String::from("Lost In Space"));
    } else if x.contains("MastersOfTheUniverse") {
        return (String::from("MastersOfTheUniverse"), String::from("Masters Of The Universe"));
    } else if x.contains("NightSky") {
        return (String::from("NightSky"), String::from("Night Sky"));
    } else if x.contains("Orville") {
        return (String::from("Orville"), String::from("Orville"));
    } else if x.contains("RaisedByWolves") {
        return (String::from("RaisedByWolves"), String::from("Raised By Wolves"));
    } else if x.contains("Silo") {
        return (String::from("Silo"), String::from("Silo"));
    } else if x.contains("StarTrek/Discovery") {
        return (String::from("Discovery"), String::from("Discovery"));
    } else if x.contains("StarTrek/Picard") {
        return (String::from("Picard"), String::from("Picard"));
    } else if x.contains("StarTrek/StrangeNewWorlds") {
        return (String::from("StrangeNewWorlds"), String::from("Strange New Worlds"));
    } else if x.contains("StarTrek/LowerDecks") {
        return (String::from("LowerDecks"), String::from("Lower Decks"));
    } else if x.contains("StarTrek/Prodigy") {
        return (String::from("Prodigy"), String::from("Prodigy"));
    } else if x.contains("StarTrek/TNG") {
        return (String::from("TNG"), String::from("TNG"));
    } else if x.contains("StarTrek/STTV") {
        return (String::from("STTV"), String::from("STTV"));
    } else if x.contains("StarTrek/Voyager") {
        return (String::from("Voyager"), String::from("Voyager"));
    } else if x.contains("StarTrek/Enterprise") {
        return (String::from("Enterprise"), String::from("Enterprise"));
    } else if x.contains("Andor") {
        return (String::from("Andor"), String::from("Andor"));
    } else if x.contains("BookOfBobaFett") {
        return (String::from("BobaFett"), String::from("Boba Fett"));
    } else if x.contains("Mandalorian") {
        return (String::from("Mandalorian"), String::from("Mandalorian"));
    } else if x.contains("ObiWanKenobi") {
        return (String::from("ObiWan"), String::from("Obi Wan"));
    } else if x.contains("TalesOfTheJedi") {
        return (String::from("TalesOfTheJedi"), String::from("Tales Of The Jedi"));
    } else if x.contains("TheBadBatch") {
        return (String::from("TheBadBatch"), String::from("The Bad Batch"));
    } else if x.contains("Visions") {
        return (String::from("Visions"), String::from("Visions"));
    } else if x.contains("TheLastOfUs") {
        return (String::from("TheLastOfUs"), String::from("The Last Of Us"));
    } else if x.contains("TheLordOfTheRingsTheRingsOfPower") {
        return (String::from("RingsOfPower"), String::from("Rings Of Power"));
    } else if x.contains("WheelOfTime") {
        return (String::from("WheelOfTime"), String::from("Wheel Of Time"));
    } else if x.contains("FalconWinterSoldier") {
        return (String::from("FalconWinterSoldier"), String::from("Falcon Winter Soldier"));
    } else if x.contains("Hawkeye") {
        return (String::from("Hawkeye"), String::from("Hawkeye"));
    } else if x.contains("IAmGroot") {
        return (String::from("IAmGroot"), String::from("I Am Groot"));
    } else if x.contains("Loki") {
        return (String::from("Loki"), String::from("Loki"));
    } else if x.contains("MoonKnight") {
        return (String::from("MoonKnight"), String::from("Moon Knight"));
    } else if x.contains("MSMarvel") {
        return (String::from("MSMarvel"), String::from("MS Marvel"));
    } else if x.contains("SecretInvasion") {
        return (String::from("SecretInvasion"), String::from("Secret Invasion"));
    } else if x.contains("SheHulk") {
        return (String::from("SheHulk"), String::from("She Hulk"));
    } else if x.contains("WandaVision") {
        return (String::from("WandaVision"), String::from("Wanda Vision"));
    } else if x.contains("PrehistoricPlanet") {
        return (String::from("PrehistoricPlanet"), String::from("Prehistoric Planet"));
    } else if x.contains("Ahsoka") {
        return (String::from("Ahsoka"), String::from("Ahsoka"));
    } else if x.contains("TheContinental") {
        return (String::from("TheContinental"), String::from("The Continental"));
    } else if x.contains("MonarchLegacyOfMonsters") {
        return (String::from("MonarchLegacyOfMonsters"), String::from("Monarch Legacy Of Monsters"));
    } else if x.contains("Shogun") {
        return (String::from("Shogun"), String::from("Shogun"));
    } else if x.contains("Fallout") {
        return (String::from("Fallout"), String::from("Fallout"));
    } else if x.contains("3BodyProblem") {
        return (String::from("3BodyProblem"), String::from("3 Body Problem"));
    } else {
        return (String::from("None"), String::from("None"));
    }
}

pub fn process_tvshows(tv: String, count: u32) {
    let season = get_season(&tv);
    let episode = get_episode(&tv);
    let filesize = crate::mtv_utils::get_file_size(&tv);
    let namef = get_tv_catagory_name(&tv);
    let fcat = namef.0;
    let fname = namef.1;
    let tvshow = mtv_types::TVShow {
        id: count,
        tvid: crate::mtv_utils::create_md5(&tv),
        size: filesize.to_string(),
        catagory: fcat,
        name: fname,
        season: season,
        episode: episode,
        path: tv,
        idx: count.to_string(),
    };
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute(
        "INSERT INTO tvshows (TvId, Size, Catagory, Name, Season, Episode, Path, Idx) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[&tvshow.tvid, &tvshow.size, &tvshow.catagory, &tvshow.name, &tvshow.season, &tvshow.episode, &tvshow.path, &tvshow.idx],

    )
    .expect("Unable to insert new tvshow info");
    // println!("inserted: {:?}", tvshow);
}
