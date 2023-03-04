use glob::glob;
use std::env;
use std::fs;

fn clean_movie_meta_dir() {
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
}

fn clean_music_meta_dir() {
    let music_meta_dir_path = env::var("MTV_MUSIC_METADATA_PATH").unwrap();
    let glob_str = music_meta_dir_path + "/*.json";
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        let rm_path = e.unwrap();

        println!("{:?}", rm_path);
        fs::remove_file(rm_path).expect("File delete failed");
        println!("File deleted successfully!");
    }
}

pub fn clean_meta() {
    clean_movie_meta_dir();
    clean_music_meta_dir();
}
