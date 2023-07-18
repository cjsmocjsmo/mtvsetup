use std::env;
use std::time::Instant;

// mod mtv_clean;
// mod mtv_create_ids;
mod mtv_image;
mod mtv_misc;
mod mtv_process_movie_images;
mod mtv_process_movies;
mod mtv_process_tvshows;
mod mtv_split;
mod mtv_tables;
pub mod mtv_types;
mod mtv_walk_dirs;

pub fn setup() -> bool {
    let start = Instant::now();

    let _tables = mtv_tables::create_tables();

    let usb1 = env::var("MTV_USB1").expect("$MTV_USB1 is not set");
    let usb2 = env::var("MTV_USB2").expect("$MTV_USB2 is not set");
    let usb3 = env::var("MTV_USB3").expect("$MTV_USB3 is not set");
    let usb4 = env::var("MTV_USB4").expect("$MTV_USB4 is not set");

    let usblist = Vec::from([usb1, usb2, usb3, usb4]);

    for usb in usblist {
        if usb != "None" {
            let usb_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb.clone());

            let usb_thumbnailz = usb_movies_vec_vec[2].clone();
            if usb_thumbnailz.clone().len() > 0 {
                let mut count = 0;
                for thumb in usb_thumbnailz {
                    count = count + 1;
                    let _process_movie_posters = mtv_process_movie_images::process_movie_posters(
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
                    let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
                    println!("{}", mov.clone());
                }
            }

            let usb_tvshowz = usb_movies_vec_vec[1].clone();
            if usb_tvshowz.clone().len() > 0 {
                let mut count = 0;
                for tv in usb_tvshowz {
                    count = count + 1;
                    let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
                    // println!("{}", tv.clone());
                }
            }
        }
    }
    println!("MTV Setup time is: {:?}", start.elapsed());

    true
}

//     let usb1_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb1.clone());

//     let usb1_thumbnailz = usb1_movies_vec_vec[2].clone();
//     if usb1_thumbnailz.clone().len() > 0 {
//         let mut count = 0;
//         for thumb in usb1_thumbnailz {
//             count = count + 1;
//             let _process_movie_posters =
//                 mtv_process_movie_images::process_movie_posters(thumb.clone(), count.clone());
//             println!("thumb count: {}", count.clone());
//         }
//     }

//     let usb1_moviez = usb1_movies_vec_vec[0].clone();
//     if usb1_moviez.clone().len() > 0 {
//         let mut count = 0;
//         for mov in usb1_moviez {
//             count = count + 1;
//             let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
//             println!("{}", mov.clone());
//         }
//     }

//     let usb1_tvshowz = usb1_movies_vec_vec[1].clone();
//     if usb1_tvshowz.clone().len() > 0 {
//         let mut count = 0;
//         for tv in usb1_tvshowz {
//             count = count + 1;
//             let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
//             // println!("{}", tv.clone());
//         }
//     }
// }

// if usb1 != "None" {
//     let usb1_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb1.clone());

//     let usb1_thumbnailz = usb1_movies_vec_vec[2].clone();
//     if usb1_thumbnailz.clone().len() > 0 {
//         let mut count = 0;
//         for thumb in usb1_thumbnailz {
//             count = count + 1;
//             let _process_movie_posters =
//                 mtv_process_movie_images::process_movie_posters(thumb.clone(), count.clone());
//             println!("thumb count: {}", count.clone());
//         }
//     }

//     let usb1_moviez = usb1_movies_vec_vec[0].clone();
//     if usb1_moviez.clone().len() > 0 {
//         let mut count = 0;
//         for mov in usb1_moviez {
//             count = count + 1;
//             let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
//             println!("{}", mov.clone());
//         }
//     }

//     let usb1_tvshowz = usb1_movies_vec_vec[1].clone();
//     if usb1_tvshowz.clone().len() > 0 {
//         let mut count = 0;
//         for tv in usb1_tvshowz {
//             count = count + 1;
//             let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
//             // println!("{}", tv.clone());
//         }
//     }
// }

//     let usb2 = env::var("MTV_USB2").expect("$MTV_USB2 is not set");

//     if usb2 != "None" {
//         let usb2_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb2.clone());

//         let usb2_thumbnailz = usb2_movies_vec_vec[2].clone();
//         if usb2_thumbnailz.clone().len() > 0 {
//             let mut count = 0;
//             for thumb in usb2_thumbnailz {
//                 count = count + 1;
//                 let _process_movie_posters =
//                     mtv_process_movie_images::process_movie_posters(thumb.clone(), count.clone());
//                 println!("thumb count: {}", count.clone());
//             }
//         }

//         let usb2_moviez = usb2_movies_vec_vec[0].clone();
//         if usb2_moviez.clone().len() > 0 {
//             let mut count = 0;
//             for mov in usb2_moviez {
//                 count = count + 1;
//                 let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
//                 println!("{}", mov.clone());
//             }
//         }

//         let usb2_tvshowz = usb2_movies_vec_vec[1].clone();
//         if usb2_tvshowz.clone().len() > 0 {
//             let mut count = 0;
//             for tv in usb2_tvshowz {
//                 count = count + 1;
//                 let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
//                 // println!("{}", tv.clone());
//             }
//         }
//     }

//     let usb3 = env::var("MTV_USB3").expect("$MTV_USB3 is not set");

//     if usb3 != "None" {
//         let usb3_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb3.clone());

//         let usb3_thumbnailz = usb3_movies_vec_vec[2].clone();
//         if usb3_thumbnailz.clone().len() > 0 {
//             let mut count = 0;
//             for thumb in usb3_thumbnailz {
//                 count = count + 1;
//                 let _process_movie_posters =
//                     mtv_process_movie_images::process_movie_posters(thumb.clone(), count.clone());
//                 println!("thumb count: {}", count.clone());
//             }
//         }

//         let usb3_moviez = usb3_movies_vec_vec[0].clone();
//         if usb3_moviez.clone().len() > 0 {
//             let mut count = 0;
//             for mov in usb3_moviez {
//                 count = count + 1;
//                 let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
//                 println!("{}", mov.clone());
//             }
//         }

//         let usb3_tvshowz = usb3_movies_vec_vec[1].clone();
//         if usb3_tvshowz.clone().len() > 0 {
//             let mut count = 0;
//             for tv in usb3_tvshowz {
//                 count = count + 1;
//                 let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
//                 // println!("{}", tv.clone());
//             }
//         }
//     }

//     let usb4 = env::var("MTV_USB4").expect("$MTV_USB4 is not set");

//     if usb4 != "None" {
//         let usb4_movies_vec_vec = mtv_walk_dirs::walk_movies_dir(usb4.clone());

//         let usb4_thumbnailz = usb4_movies_vec_vec[2].clone();
//         if usb4_thumbnailz.clone().len() > 0 {
//             let mut count = 0;
//             for thumb in usb4_thumbnailz {
//                 count = count + 1;
//                 let _process_movie_posters =
//                     mtv_process_movie_images::process_movie_posters(thumb.clone(), count.clone());
//                 println!("thumb count: {}", count.clone());
//             }
//         }

//         let usb4_moviez = usb4_movies_vec_vec[0].clone();
//         if usb4_moviez.clone().len() > 0 {
//             let mut count = 0;
//             for mov in usb4_moviez {
//                 count = count + 1;
//                 let _process_movies = mtv_process_movies::process_movies(mov.clone(), count);
//                 println!("{}", mov.clone());
//             }
//         }

//         let usb4_tvshowz = usb4_movies_vec_vec[1].clone();
//         if usb4_tvshowz.clone().len() > 0 {
//             let mut count = 0;
//             for tv in usb4_tvshowz {
//                 count = count + 1;
//                 let _process_tvshows = mtv_process_tvshows::process_tvshows(tv.clone(), count);
//                 // println!("{}", tv.clone());
//             }
//         }
//     }
//     println!("MTV Setup time is: {:?}", start.elapsed());

//     true
// }
