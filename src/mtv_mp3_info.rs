use id3::{Tag, TagLike};
use json::object;
use mp3_duration;
use std::env;
use std::fs;
use std::path::Path;

pub fn get_tag_info(x: &String) -> (String, String, String, String) {
    let tag = Tag::read_from_path(x).unwrap();
    let artist = tag.artist().unwrap().to_string();
    let album = tag.album().unwrap().to_string();
    let song = tag.title().unwrap().to_string();
    let genre = tag.genre().unwrap().to_string();

    (artist, album, song, genre)
}

pub fn get_duration(x: &String) -> String {
    let path = Path::new(&x);
    let dur_sec = mp3_duration::from_path(&path).expect("this is duration exception");
    if format!("{:?}", dur_sec) == "this is duration exception".to_string() {
        println!("{}", x);
    }

    let dur_min = dur_sec.div_f32(60.0);
    let dur_str = format!("{:?}", dur_min);
    let mut durvec = vec![];
    for i in dur_str.chars() {
        durvec.push(i);
    }

    let mut newvec = vec![];
    let mut count: i32 = 0;
    for c in durvec {
        count = count + 1;
        if count < 5 {
            newvec.push(c);
        } else {
            break;
        };
    }

    let duration: String = newvec.into_iter().collect();

    duration
}

pub fn split_sep1(x: String) -> Vec<String> {
    let xsplit = x.split("_");
    let mut xvec = vec![];
    for word in xsplit {
        xvec.push(word.to_string());
    }

    xvec
}

pub fn split_sep2(x: String) -> Vec<String> {
    let yslit = x.split(" ");
    let mut yvec = vec![];
    for word in yslit {
        yvec.push(word.to_string());
    }

    yvec
}

pub fn split_sep3(x: String) -> Vec<String> {
    let filesplit = x.split("_-_");
    let mut fvec = vec![];
    for file in filesplit {
        fvec.push(file.to_string());
    }

    fvec
}

pub fn check_artist(x: &String, y: &String) -> bool {
    let f = split_sep1((&x).to_string());
    let t = split_sep2((&y).to_string());
    if f != t {
        return false;
    } else {
        return true;
    }
}

pub fn check_album(x: &String, y: &String) -> bool {
    let f = split_sep1((&x).to_string());
    let t = split_sep2((&y).to_string());
    if f != t {
        return false;
    } else {
        return true;
    }
}

pub fn check_song(f: &String, s: &String) -> bool {
    let mut xx = split_sep3((&f).to_string());
    let count = xx.len() - 1;
    xx.drain(0..count);
    let yy = split_sep2((&s).to_string());
    let mut pussy = false;
    for x in xx {
        let fuck = split_sep1(x.to_string());
        if yy == fuck {
            pussy = true;
        } else {
            pussy = false;
        }
    }

    pussy
}

pub fn write_music_json_to_file(
    id: String,
    voodoo: String,
    artist: String,
    album: String,
    song: String,
    genre: String,
    base_dir: String,
    filename_results: String,
    music_artist_results: String,
    music_album_results: String,
    duration_results: String,
    artc: bool,
    albc: bool,
    sc: bool,
    fullpath: String,
    extension: String,
    idx: String,
    fsize_results: String,
) {
    let mut named_incorrectly_vec = vec![];

    // println!("{}", artc);
    // println!("{}", albc);
    // println!("{}", sc);

    if artc == true && albc == true && sc == true {
        println!("\n they all match:\n {}", fullpath);

        let mp3_info = object! {
            mp3id: id,
            fullpath: fullpath,
            basedir: base_dir,
            filename: filename_results,
            ext: extension,
            imgurl: &*voodoo,
            mp3_url: &*voodoo,
            tag_artist: artist,
            tag_album: album,
            tag_title: song,
            tag_genre: genre,
            idx: idx.clone(),
            fsize: fsize_results,
            filename_artist: &*music_artist_results,
            filename_album: &*music_album_results,
            duration: duration_results,
        };

        let mfo: String = json::stringify(mp3_info.dump());

        let mtv_music_metadata_path =
            env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

        let a = format!("{}/", mtv_music_metadata_path.as_str());
        let b = format!("Music_File_Meta_{}.json", &idx);
        let outpath = a + &b;
        std::fs::write(outpath, mfo.clone()).unwrap();

        // println!("\n\n\n mp3info {}", mfo.clone());
    } else {
        
        named_incorrectly_vec.push(fullpath.as_str());
        println!("This is named incorrectly: \n {}", fullpath.as_str());
    }

    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

    let a = format!("{}/", mtv_music_metadata_path.as_str());
    let b = format!("Named_Incorrectly.json");
    let outpath = a + &b;
    fs::write(outpath, named_incorrectly_vec.join("\n"))
        .expect("Failed to write named incorrectly json file");
    println!("There are {} mp3s", &idx);
}
