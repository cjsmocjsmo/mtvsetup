use md5::{Digest, Md5};
use walkdir::WalkDir;
use filesize::PathExt;
use std::path::Path;
use byte_unit::Byte;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;

pub fn get_md5(astring: &String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(&astring);
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

pub fn get_file_size(x: &String) -> u64 {
    let path = Path::new(&x);

    path.size_on_disk().unwrap()
}

pub fn media_total_size(addr: String) -> String {
    let total_size = WalkDir::new(addr)
        .min_depth(1)
        .max_depth(5)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    let btos = total_size.to_string();
    let result = Byte::from_str(btos).unwrap();
    let size = result.get_appropriate_unit(true).to_string();

    size
}

pub fn write_music_gzip_file() -> Result<(), std::io::Error> {
    let music_meta = env::var("MTV_MUSIC_METADATA_PATH").unwrap();
    let static_path = env::var("MTV_STATIC_PATH").unwrap();
    let new_music_backup_path = static_path + "/MTV_Music_Backup.tar.gz";
    
    let tar_gz = File::create(new_music_backup_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("Music_Backups/", music_meta)?;
    Ok(())
}

pub fn write_movie_gzip_file() -> Result<(), std::io::Error> {
    let movie_meta = env::var("MTV_MOVIES_METADATA_PATH").unwrap();
    let static_path = env::var("MTV_STATIC_PATH").unwrap();
    let new_moviec_backup_path = static_path + "/MTV_Movie_Backup.tar.gz";

    let tar_gz = File::create(new_moviec_backup_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("Movie_Backups/", movie_meta)?;
    Ok(())
}