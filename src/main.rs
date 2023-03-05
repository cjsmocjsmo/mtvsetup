use std::env;

mod mtv_clean;
mod mtv_env_vars;
mod mtv_image;
mod mtv_misc;
mod mtv_mp3_info;
mod mtv_process_music_images;
mod mtv_split;
mod mtv_walk_dirs;
mod mtv_process_mp3s;

fn main() {
    let dockervar = mtv_env_vars::get_docker_var();
    if dockervar != "DOCKER".to_string() {
        mtv_env_vars::read_config();
        println!(
            "this is MTV_DOCKER_VAR :\n {}",
            env::var("MTV_DOCKER_VAR").unwrap()
        );
    } else {
        println!("docker var is set so docker will set the env vars for us");
    };

    // mtv_env_vars::read_config();

    mtv_clean::clean_meta();

    mtv_process_music_images::process_music_images();

    mtv_process_mp3s::process_mp3s();


    // let _movievec = mtv_walk_dirs::walk_movies_dir();
    // let _moviethumbvec = mtv_walk_dirs::walk_movies_thumb_dir();

    // let _music_metadata = mtv_walk_dirs::walk_metadata_music();
    // let _movies_metadata = mtv_walk_dirs::walk_metadata_movies();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    println!(
        "Total size: {} .",
        mtv_misc::media_total_size(mtv_media_path)
    );
}

// fn mtv_setup() -> String {
//     let dockervar = mtvserver::get_docker_var();
//     if dockervar == "docker var not set".to_string() {
//         mtvserver::set_all_env_vars();
//         println!(
//             "should be /media/charliepi/FOO/media :\n {}",
//             env::var("MTV_MEDIA_PATH").unwrap()
//         );
//     } else {
//         println!("docker var is set so docker will set the env vars for us");
//     };

//     // mtvserver::clean_meta();

//     process_music_images();

//     process_mp3s();

//     "Setup Complete".to_string()
// }
