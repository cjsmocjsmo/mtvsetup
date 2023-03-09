use glob::glob;
use std::env;
use std::fs;

fn clean_movie_meta_dir() -> u32 {
    let movie_meta_dir_path = env::var("MTV_MOVIES_METADATA_PATH").unwrap();
    let glob_str = movie_meta_dir_path + "/*.json";
    let mut count = 0;
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        let rm_path = e.unwrap();
        count = count + 1;

        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        fs::remove_file(rm_path).expect("File delete failed");
        println!("{} Files have been deleted successfully!", count);
    }

    count
}

fn clean_music_meta_dir() -> u32{
    let music_meta_dir_path = env::var("MTV_MUSIC_METADATA_PATH").unwrap();
    let glob_str = music_meta_dir_path + "/*.json";
    let mut count = 0;
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        count = count + 1;
        let rm_path = e.unwrap();

        fs::remove_file(rm_path).expect("File delete failed");
    }
    count
}

pub fn clean_meta() -> u32{
    let mov = clean_movie_meta_dir();
    let mus = clean_music_meta_dir();
    let tot = mov + mus;

    tot
}
