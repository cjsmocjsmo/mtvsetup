use crate::envvars;
use crate::setup::mtv_utils::db_check_file_exists;
use std::env;
use std::time::Instant;

pub mod mtv_image;
mod mtv_process_movies;
mod mtv_process_tvshows;
pub mod mtv_tables;
pub mod mtv_types;
pub mod mtv_utils;
mod mtv_walk_dirs;

pub fn setup() -> bool {
    let start = Instant::now();

    let _vars = envvars::set_env_vars();

    if !db_check_file_exists() {

        let _tables = mtv_tables::create_db_file();
        let _gen_db_check_file = mtv_utils::gen_db_check_file();

        // if mtv_image::thumbnail_dir_exists() == false {
        //     mtv_image::create_thumbnail_dir();
        // }

        let _tables = mtv_tables::create_tables();

        let usb1 = env::var("MTV_USB1").expect("$MTV_USB1 is not set");
        let usb2 = env::var("MTV_USB2").expect("$MTV_USB2 is not set");
        let usb3 = env::var("MTV_USB3").expect("$MTV_USB3 is not set");
        let usb4 = env::var("MTV_USB4").expect("$MTV_USB4 is not set");

        let usblist = Vec::from([usb1, usb2, usb3, usb4]);

        println!("usblist: {:?}", usblist.clone());

        for usb in usblist {
            if usb != "None" {
                let usb_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb.clone());

                let usb_thumbnailz = usb_movies_vec_vec[2].clone();
                if usb_thumbnailz.clone().len() > 0 {
                    let mut count = 0;
                    for thumb in usb_thumbnailz {
                        count = count + 1;
                        let _process_movie_posters = crate::setup::mtv_image::process_movie_posters(
                            thumb.clone(),
                            count.clone(),
                        );
                        println!("thumb count: {}", count.clone());
                    }
                }

                let usb_moviez = usb_movies_vec_vec[0].clone();
                if usb_moviez.clone().len() > 0 {
                    let mut count = 0;
                    for mov in usb_moviez {
                        count = count + 1;
                        let _process_movies =
                            mtv_process_movies::process_movies(mov.clone(), count);
                    }
                    println!("movie count: {}", count.clone());
                }

                let usb_tvshowz = usb_movies_vec_vec[1].clone();
                if usb_tvshowz.clone().len() > 0 {
                    let mut count = 0;
                    for tv in usb_tvshowz {
                        count = count + 1;
                        let _process_tvshows =
                            mtv_process_tvshows::process_tvshows(tv.clone(), count);
                    }
                    println!("tvshow count: {}", count.clone());
                }
            }
        }
        // let setup_time = println!("MTV Setup time is: {:?}", start.elapsed());

        let _write_file = crate::setup::mtv_utils::write_current_datetime_to_file(start.elapsed());
    } else {
        println!("Database file exists, skipping setup");
    }

    true
}
