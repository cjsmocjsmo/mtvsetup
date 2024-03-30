use walkdir::WalkDir;

pub fn walk_movies_dir(mtv_movies_path: String) -> Vec<String> {
    let mut moviesvec = Vec::new();
    for e in WalkDir::new(mtv_movies_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            println!("fname: {:?}", fname.clone());
            if fname.contains("Movies") {
                if fname.ends_with(".mp4") {
                    moviesvec.push(fname.clone());
                }
            }
        }
    }

    moviesvec
}


pub fn walk_tvshows_dir(mtv_tvshows_path: String) -> Vec<String> {
    let mut tvshowsvec = Vec::new();
    for e in WalkDir::new(mtv_tvshows_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            println!("fname: {:?}", fname.clone());
            if fname.contains("TVShows") {
                if fname.ends_with(".mp4") {
                    tvshowsvec.push(fname.clone());
                }
            }
        }
    }

    tvshowsvec
}

pub fn walk_posters_dir(mtv_poster_path: String) -> Vec<String> {
    let mut thumb_vec = Vec::new();
    for e in WalkDir::new(mtv_poster_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            println!("fname: {:?}", fname.clone());
            if fname.contains("Posters") {
                if fname.ends_with(".jpg") {
                    thumb_vec.push(fname);
                }
            }
        }
    }
    
    thumb_vec
}
