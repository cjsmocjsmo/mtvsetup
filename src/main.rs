use std::env;
use std::time::Instant;

mod mtv_clean;
mod mtv_create_ids;
mod mtv_env_vars;
mod mtv_image;
mod mtv_misc;
mod mtv_mp3_info;
mod mtv_nnc_info;
mod mtv_process_mp3s;
mod mtv_process_music_images;
mod mtv_split;
mod mtv_walk_dirs;

fn main() {
    let start = Instant::now();
    let dockervar = mtv_env_vars::get_docker_var();
    if dockervar != "DOCKER".to_string() {
        mtv_env_vars::read_config();
        println!(
            "this is MTV_DOCKER_VAR: {}",
            env::var("MTV_DOCKER_VAR").unwrap()
        );
    } else {
        println!("docker var is set so docker will set the env vars for us");
    };

    mtv_clean::clean_meta();

    mtv_process_music_images::process_music_images();

    let not_named_correctly = mtv_process_mp3s::process_mp3s();

    mtv_nnc_info::gather_media_info(not_named_correctly);

    mtv_create_ids::create_artist_id_list();

    mtv_create_ids::create_album_id_list();

    mtv_misc::write_music_gzip_file().unwrap();

    mtv_misc::write_movie_gzip_file().unwrap();

    // let _movievec = mtv_walk_dirs::walk_movies_dir();
    // let _moviethumbvec = mtv_walk_dirs::walk_movies_thumb_dir();

    // let _music_metadata = mtv_walk_dirs::walk_metadata_music();
    // let _movies_metadata = mtv_walk_dirs::walk_metadata_movies();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    println!(
        "Total size: {} .",
        mtv_misc::media_total_size(mtv_media_path)
    );
    let duration = start.elapsed();

    println!("MTV Setup time is: {:?}", duration);
}
