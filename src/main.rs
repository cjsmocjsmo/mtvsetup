use std::env;
use std::time::Instant;

mod mtv_clean;
mod mtv_create_ids;
mod mtv_env_vars;
mod mtv_image;
mod mtv_misc;
mod mtv_mp3_info;
mod mtv_nnc_info;
mod mtv_process_music;
mod mtv_process_music_images;
mod mtv_split;
mod mtv_walk_dirs;

fn main() {
    let start = Instant::now();
    let dockervar = mtv_env_vars::get_docker_var();
    if dockervar != "DOCKER".to_string() {
        mtv_env_vars::read_config();
    } else {
        println!("docker var is set so docker will set the env vars for us");
    };

    // println!("{}", "\nStarting Clean");
    // let tot = mtv_clean::clean_meta();

    // println!("{}", "\nStarting Image Processing");
    // mtv_process_music_images::process_music_images();

    // println!("{}", "\nStarting Music Processing");
    // mtv_process_mp3s::process_mp3s();

    // println!("{}", "\nStarting Create Artist Id Started");
    // mtv_create_ids::create_artist_id_list();

    // println!("{}", "\nStarting Create Album Id Started");
    // mtv_create_ids::create_album_id_list();

    // println!("{}", "\nStarting Music Gzip");
    // mtv_misc::write_music_gzip_file().unwrap();

    // println!("{}", "\nStarting Movie Gip");
    // mtv_misc::write_movie_gzip_file().unwrap();

    let movies_vec = mtv_walk_dirs::walk_movies_dir();
    // let movie_posters_vec = mtv_walk_dirs::walk_posters2_dir();

    // let _music_metadata = mtv_walk_dirs::walk_metadata_music();
    // let _movies_metadata = mtv_walk_dirs::walk_metadata_movies();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    // println!("Clean has removed {} files.", tot);
    println!("{}", "Artist Id List has been wirtten");
    println!("{}", "Album Id List has been wirtten");
    println!(
        "Total size: {} .",
        mtv_misc::media_total_size(mtv_media_path)
    );
    let duration = start.elapsed();

    println!("MTV Setup time is: {:?}", duration);
}
