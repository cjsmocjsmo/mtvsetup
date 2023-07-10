use std::env;
use std::time::Instant;

// mod mtv_clean;
mod mtv_create_ids;
mod mtv_env_vars;
mod mtv_image;
mod mtv_misc;
// mod mtv_mp3_info;
// mod mtv_nnc_info;
mod mtv_process_movie_images;
mod mtv_process_movies;
// mod mtv_process_music;
// mod mtv_process_music_images;
mod mtv_process_tvshows;
mod mtv_split;
mod mtv_walk_dirs;
mod mtv_types;
mod mtv_tables;



fn main() {
    let start = Instant::now();

    let _tables = mtv_tables::create_tables();

    // let mut usbpaths = Vec::new();
    let usb1 = env::var("MTV_USB1").expect("$MTV_USB1 is not set");
    println!("{}", "\nStarting Movie processing");
    let _processed_movies_count = mtv_process_movies::process_movies(usb1);
    

    // let usb2 = env::var("MTV_USB2").expect("$MTV_USB2 is not set");
    
    // let usb3 = env::var("MTV_USB3").expect("$MTV_USB3 is not set");
    
    // let usb4 = env::var("MTV_USB4").expect("$MTV_USB4 is not set");
   

    

    // let dockervar = mtv_env_vars::get_docker_var();
    // if dockervar != "DOCKER".to_string() {
    //     mtv_env_vars::read_config();
    // } else {
    //     println!("docker var is set so docker will set the env vars for us");
    // };

    // println!("{}", "\nStarting Clean");
    // let tot = mtv_clean::clean_meta();

    // println!("{}", "\nStarting Image Processing");
    // let music_img_count = mtv_process_music_images::process_music_images();

    // println!("{}", "\nStarting Music Processing");
    // let music_count = mtv_process_music::process_mp3s();

    // println!("{}", "\nStarting Create Artist Id");
    // mtv_create_ids::create_artist_id_list();

    // println!("{}", "\nStarting Create Album Id");
    // mtv_create_ids::create_album_id_list();

    

    // println!("{}", "\nStarting Posters2 processing");
    // let processed_movie_posters = mtv_process_movie_images::process_movie_posters();

    // println!("{}", "\nStarting TVShows processing");
    // let processed_tvshow_count = mtv_process_tvshows::process_tvshows();

    // println!("{}", "\nStarting Music Gzip");
    // mtv_misc::write_music_gzip_file().unwrap();
    
    // println!("{}", "\nStarting Movie Gzip");
    // mtv_misc::write_movie_gzip_file().unwrap();
    
    // println!("{}", "\nStarting TVShows Gzip");
    // mtv_misc::write_tvshows_gzip_file().unwrap();

    // println!("{}", "\nStarting Copy gzip files to https static folder");
    // let copied_count = mtv_misc::copy_gzip_files();

    let mtv_media_path = env::var("MTV_MEDIA_PATH").expect("$MTV_MEDIA_PATH is not set");

    // println!("\nClean has removed {} files.", tot);
    // println!("Music Processed: {} files", music_count);
    // println!("Music Images Processed: {} files", music_img_count.0);
    // println!("Movies Processed: {} files", processed_movies_count);
    // println!("TVShows Processed: {} files", processed_tvshow_count);
    // println!(
    //     "Music Posters Processed: {} files",
    //     processed_movie_posters.1
    // );
    // println!("{} music images failed to open", music_img_count.1);
    // println!("{} movie posters failed to open", processed_movie_posters.0);
    println!("{}", "Artist Id List has been wirtten");
    println!("{}", "Album Id List has been wirtten");
    // println!("{} files copied to static", copied_count);


    println!(
        "Total size: {} .",
        mtv_misc::media_total_size(mtv_media_path)
    );

    println!("MTV Setup time is: {:?}", start.elapsed());
}
