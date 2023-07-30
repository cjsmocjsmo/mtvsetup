use walkdir::WalkDir;

pub fn walk_movies_dir(mtv_movies_path: String) -> Vec<Vec<String>> {
    let mut moviesvec = Vec::new();
    let mut mov_tv_thumb_vec = Vec::new();
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
                } else if fname.ends_with(".m4v") {
                    moviesvec.push(fname.clone());
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
            } else if fname.contains("MovPosters") {
                if fname.ends_with(".jpg") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".jpeg") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".png") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".webp") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".avif") {
                    mov_tv_thumb_vec.push(fname);
                } else {
                    continue;
                }
            } else if fname.contains("Posters2") {
                if fname.ends_with(".jpg") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".jpeg") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".png") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".webp") {
                    mov_tv_thumb_vec.push(fname);
                } else if fname.ends_with(".avif") {
                    mov_tv_thumb_vec.push(fname);
                } else {
                    continue;
                }
            } else if fname.contains("Music") {
                continue;
            } else {
                println!(
                    "fname does not contain Movies, TVShows or Music: \n\t{}",
                    fname.clone()
                )
            }
        }
    }
    let media_vec = vec![moviesvec, tvshowsvec, mov_tv_thumb_vec];

    media_vec
}

