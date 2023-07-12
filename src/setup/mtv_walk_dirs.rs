use walkdir::WalkDir;

pub fn walk_movies_dir(mtv_movies_path: String) -> Vec<Vec<String>> {
    let mut moviesvec = Vec::new();
    let mut moviesthumbvec = Vec::new();
    let mut tvshowsvec = Vec::new();
    for e in WalkDir::new(mtv_movies_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.contains("Movies") {
                if fname.ends_with(".mp4") {
                    moviesvec.push(fname.clone());
                } else if fname.ends_with(".mkv") {
                    moviesvec.push(fname.clone());
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
            } else if fname.contains("TVShows") {
                if fname.ends_with(".mp4") {
                    tvshowsvec.push(fname.clone());
                } else if fname.ends_with(".mkv") {
                    tvshowsvec.push(fname.clone());
                } else {
                    continue;
                }
            } else {
                println!(
                    "fname does not contain Movies or TVShows: \n\t{}",
                    fname.clone()
                )
            }
        }
    }
    let media_vec = vec![moviesvec, tvshowsvec, moviesthumbvec];

    media_vec
}

// pub fn walk_movies_dir(mtv_movies_path: String) -> Vec<Vec<String>> {
//     let mut moviesvec = Vec::new();
//     let mut moviesthumbvec = Vec::new();
//     let mut tvshowsvec = Vec::new();
//     for e in WalkDir::new(mtv_movies_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();
//             if fname.contains("Movies") {
//                 if fname.ends_with(".mp4") {
//                     if fname.contains("TVShows") {
//                         tvshowsvec.push(fname.clone());
//                     } else {
//                         moviesvec.push(fname.clone());
//                     }
//                 } else if fname.ends_with(".mkv") {
//                     if fname.contains("TVShows") {
//                         tvshowsvec.push(fname.clone());
//                     } else {
//                         moviesvec.push(fname.clone());
//                     }
//                 } else if fname.ends_with(".jpg") {
//                     moviesthumbvec.push(fname);
//                 } else if fname.ends_with(".jpeg") {
//                     moviesthumbvec.push(fname);
//                 } else if fname.ends_with(".png") {
//                     moviesthumbvec.push(fname);
//                 } else if fname.ends_with(".webp") {
//                     moviesthumbvec.push(fname);
//                 } else if fname.ends_with(".avif") {
//                     moviesthumbvec.push(fname);
//                 } else {
//                     continue;
//                 }
//             }
//         }
//     }
//     let media_vec = vec![moviesvec, tvshowsvec, moviesthumbvec];

//     media_vec
// }
