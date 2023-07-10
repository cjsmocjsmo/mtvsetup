// use std::env;
use walkdir::WalkDir;

// pub fn walk_music_dir_music() -> Vec<String> {
//     let mut mp3vec = Vec::new();
//     let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
//     for e in WalkDir::new(mtv_music_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();

//             if fname.ends_with(".mp3") {
//                 mp3vec.push(fname.clone());
//             } else {
//                 continue;
//             }
//         }
//     }

//     mp3vec
// }

// pub fn walk_music_dir_images() -> Vec<String> {
//     let mut musicimagevec = Vec::new();
//     let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
//     for e in WalkDir::new(mtv_music_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();

//             if fname.ends_with(".jpg") {
//                 musicimagevec.push(fname);
//             } else if fname.ends_with(".jpeg") {
//                 musicimagevec.push(fname);
//             } else if fname.ends_with(".png") {
//                 musicimagevec.push(fname);
//             } else if fname.ends_with(".webp") {
//                 musicimagevec.push(fname);
//             } else if fname.ends_with(".avif") {
//                 musicimagevec.push(fname);
//             } else {
//                 continue;
//             }
//         }
//     }

//     musicimagevec
// }

pub fn walk_movies_dir(mtv_movies_path: String) -> Vec<Vec<String>> {
    let mut moviesvec = Vec::new();
    let mut moviesthumbvec = Vec::new();
    let mut tvshowsvec = Vec::new();
    // let mtv_movies_path = env::var("MTV_MOVIES_PATH").expect("$MTV_MOVIES_PATH is not set");
    for e in WalkDir::new(mtv_movies_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            // println!("this is movie file:\n\t{}\n", fname.clone());
            if fname.ends_with(".mp4") {
                if fname.contains("TVShows") {
                    tvshowsvec.push(fname.clone());
                } else {
                    moviesvec.push(fname.clone());
                }
            } else if fname.ends_with(".mkv") {
                if fname.contains("TVShows") {
                    tvshowsvec.push(fname.clone());
                } else {
                    moviesvec.push(fname.clone());
                }
            } else if fname.ends_with(".jpg") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".jpeg") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".png") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".webp") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".avif") {
                moviesthumbvec.push(fname);
            } else {
                continue;
            }
        }
    }
    let media_vec = vec![moviesvec, moviesthumbvec, tvshowsvec];

    media_vec
}

// pub fn walk_posters2_dir() -> Vec<String> {
//     let mut moviesthumbvec = Vec::new();
//     let mtv_movies_thumb_path =
//         env::var("MTV_MOVIES_POSTERS_PATH").expect("$MTV_MOVIES_POSTERS_PATH is not set");
//     for e in WalkDir::new(mtv_movies_thumb_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();
//             if fname.ends_with(".jpg") {
//                 moviesthumbvec.push(fname);
//             } else if fname.ends_with(".jpeg") {
//                 moviesthumbvec.push(fname);
//             } else if fname.ends_with(".png") {
//                 moviesthumbvec.push(fname);
//             } else if fname.ends_with(".webp") {
//                 moviesthumbvec.push(fname);
//             } else if fname.ends_with(".avif") {
//                 moviesthumbvec.push(fname);
//             } else {
//                 continue;
//             }
//         }
//     }

//     moviesthumbvec
// }

// pub fn walk_tvshows_dir() -> Vec<String> {
//     let mut tvshowsvec = Vec::new();
//     let mtv_tvshows_path = env::var("MTV_TVSHOWS_PATH").expect("$MTV_TVSHOWS_PATH is not set");
//     for e in WalkDir::new(mtv_tvshows_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();

//             if fname.ends_with(".mp4") {
//                 tvshowsvec.push(fname);
//             } else if fname.ends_with(".mkv") {
//                 tvshowsvec.push(fname);
//             } else {
//                 continue;
//             }
//         }
//     }

//     tvshowsvec
// }
