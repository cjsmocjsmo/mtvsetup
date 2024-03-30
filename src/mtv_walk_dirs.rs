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
            if fname.contains("Movies") && fname.ends_with(".mp4") {
                moviesvec.push(fname.clone());
            }
            if fname.contains("TVShows") && fname.ends_with(".mp4") {
                tvshowsvec.push(fname.clone());
            }
            if fname.contains("Posters") && fname.ends_with(".jpg") {
                mov_tv_thumb_vec.push(fname);
            }
        }
    }
    let mut fsizevec = Vec::new();
    for m in moviesvec.clone() {
        let fsize = crate::mtv_utils::get_file_size(&m);
        fsizevec.push(fsize);
    }

    for tv in tvshowsvec.clone() {
        let fsize = crate::mtv_utils::get_file_size(&tv);
        fsizevec.push(fsize);
    }

    for img in mov_tv_thumb_vec.clone() {
        let fsize = crate::mtv_utils::get_file_size(&img);
        fsizevec.push(fsize);
    }

    let mut fsizevecsum = Vec::new();
    let fsizevecsumvecc = fsizevec.iter().sum::<u64>().to_string();
    fsizevecsum.push(fsizevecsumvecc);

    let media_vec = vec![moviesvec.clone(), tvshowsvec, mov_tv_thumb_vec, fsizevecsum];

    media_vec
}
