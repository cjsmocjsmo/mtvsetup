use std::env;
use std::time::Instant;

mod mtv_clean;
mod mtv_create_ids;
mod mtv_env_vars;
mod mtv_image;
mod mtv_misc;
mod mtv_mp3_info;
mod mtv_nnc_info;
mod mtv_process_movie_images;
mod mtv_process_movies;
mod mtv_process_music;
mod mtv_process_music_images;
mod mtv_process_tvshows;
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

    println!("{}", "\nStarting Clean");
    let tot = mtv_clean::clean_meta();

    println!("{}", "\nStarting Image Processing");
    mtv_process_music_images::process_music_images();

    println!("{}", "\nStarting Music Processing");
    mtv_process_music::process_mp3s();

    println!("{}", "\nStarting Create Artist Id");
    mtv_create_ids::create_artist_id_list();

    println!("{}", "\nStarting Create Album Id");
    mtv_create_ids::create_album_id_list();

    println!("{}", "\nStarting Movie processing");
    let processed_movies_count = mtv_process_movies::process_movies();

    println!("{}", "\nStarting Posters2 processing");
    let processed_movie_posters = mtv_process_movie_images::process_movie_posters();

    println!("{}", "\nStarting TVShows processing");
    let processed_tvshow_count = mtv_process_tvshows::process_tvshows();

    println!("{}", "\nStarting Music Gzip");
    mtv_misc::write_music_gzip_file().unwrap();

    println!("{}", "\nStarting Movie Gzip");
    mtv_misc::write_movie_gzip_file().unwrap();

    println!("{}", "\nStarting TVShows Gzip");
    mtv_misc::write_tvshows_gzip_file().unwrap();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    println!("\nClean has removed {} files.", tot);
    println!("Movies Processed: {}", processed_movies_count);
    println!("There are {} bad movie posters", processed_movie_posters.0);
    println!("There are {} movie posters", processed_movie_posters.1);

    println!("{}", "Artist Id List has been wirtten");
    println!("{}", "Album Id List has been wirtten");

    println!(
        "Total size: {} .",
        mtv_misc::media_total_size(mtv_media_path)
    );
    let duration = start.elapsed();

    println!("MTV Setup time is: {:?}", duration);
}
