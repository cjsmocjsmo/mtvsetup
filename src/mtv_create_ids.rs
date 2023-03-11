use json::object;
use json::JsonValue;
use std::env;
use std::vec;

fn get_raw_artist_dir_list(dlist: Vec<String>) -> Vec<String> {
    let mut raw_dirs_vec = vec![];
    for x in dlist {
        let filesplit = x.split("/");
        let mut filenamevec = vec![];
        for file in filesplit {
            filenamevec.push(file);
        }

        filenamevec.pop();
        let item = filenamevec.join("/");
        raw_dirs_vec.push(item);
    }

    raw_dirs_vec
}

fn pop_artist(x: String) -> String {
    let mut path_vec = vec![];
    let path_split = x.split("/");
    for p in path_split {
        path_vec.push(p);
    }

    let foo = path_vec.len() - 2;
    let raw_artist = path_vec[foo];
    let sp_artist = raw_artist.split("_");

    let mut join_artist_vec = vec![];
    for sp in sp_artist {
        join_artist_vec.push(sp);
    }

    let artist = join_artist_vec.join(" ");

    artist
}

fn build_artist_id_info(path: String) -> JsonValue {
    let artist = pop_artist(path.clone());
    let artistid = crate::mtv_misc::get_md5(&path);

    object! { path: path.clone(), artist: artist, artistid: artistid }
}

pub fn create_artist_id_list() {
    let files = crate::mtv_walk_dirs::walk_music_dir_music();
    let mut raw_dirs = get_raw_artist_dir_list(files);
    raw_dirs.dedup();

    let mut id_vec = vec![];
    for d in raw_dirs {
        let entry = build_artist_id_info(d);

        id_vec.push(entry);
    }
    let info = json::stringify(id_vec);
    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

    let a = format!("{}/", mtv_music_metadata_path.as_str());
    let b = format!("Music_Artist_Ids.json");
    let outpath = a + &b;

    std::fs::write(outpath, info).unwrap();
}

fn get_raw_album_dir_list(dlist: Vec<String>) -> Vec<String> {
    let mut raw_dirs_vec = vec![];
    for x in dlist {
        let filesplit = x.split("/");
        let mut filenamevec = vec![];
        for file in filesplit {
            filenamevec.push(file);
        }

        filenamevec.pop();
        let item = filenamevec.join("/");
        raw_dirs_vec.push(item);
    }

    raw_dirs_vec
}

fn pop_album(x: String) -> String {
    let mut path_vec = vec![];
    let path_split = x.split("/");
    for p in path_split {
        path_vec.push(p);
    }

    let foo = path_vec.len() - 1;
    let raw_album = path_vec[foo];
    let ra_split = raw_album.split("/");

    let mut new_alb_list = vec![];
    for ra in ra_split {
        new_alb_list.push(ra);
    }
    let albsplit = new_alb_list[0].split("_");
    let mut newalb = vec![];
    for alb in albsplit {
        newalb.push(alb);
    }
    let foo = newalb.join(" ");

    foo.to_string()
}

pub fn create_album_id_list() {
    let files = crate::mtv_walk_dirs::walk_music_dir_music();
    let mut rawdirs = get_raw_album_dir_list(files);
    rawdirs.dedup();

    let mut newalbvec = vec![];
    for d in rawdirs {
        let album = pop_album(d.clone());
        let albpath = d.clone();
        let albumid = crate::mtv_misc::get_md5(&d);
        let album_id_info = object! {
            album: album,
            path: albpath,
            albumid: albumid
        };

        newalbvec.push(album_id_info);
    }
    let info = json::stringify(newalbvec.clone());
    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

    let a = format!("{}/", mtv_music_metadata_path.as_str());
    let b = format!("Music_Album_Ids.json");
    let outpath = a + &b;

    std::fs::write(outpath, info).unwrap();
}
