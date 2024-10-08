
// use std::env;
// use image::{self};
use rusqlite::Connection;
use std::env;

use crate::mtv_utils::create_md5;
use crate::mtv_types::MovieImage;

pub fn thumbnail_dir_exists() -> bool {
    let mtv_thumbnail_path =
        env::var("MTV_THUMBNAIL_PATH").expect("$MTV_THUMBNAIL_PATH is not set");
    let path = std::path::Path::new(&mtv_thumbnail_path);
    if path.exists() && path.is_dir() {
        return true
    } else {
        return false
    }
}

// pub fn create_thumbnail_dir() {
//     let mtv_thumbnail_path =
//         env::var("MTV_THUMBNAIL_PATH").expect("$MTV_THUMBNAIL_PATH is not set");
//     std::fs::create_dir_all(mtv_thumbnail_path).expect("Unable to create thumbnail dir");
// }

fn create_movie_thumbnail(x: String) -> (String, String) {
    let mtv_thumbnail_path =
        env::var("MTV_THUMBNAIL_PATH").expect("$MTV_THUMBNAIL_PATH is not set");
    let server_addr = env::var("MTV_SERVER_ADDR").expect("$MTV_SERVER_ADDR is not set");
    let server_port = env::var("MTV_SERVER_PORT").expect("$MTV_SERVER_PORT is not set");
    let old_fname = crate::mtv_utils::split_poster_name(x.clone());
    let out_fname = mtv_thumbnail_path + &old_fname;
    println!("out_fname: {:?}", out_fname);
    let http_fname = server_addr + ":" + &server_port + "/" + &old_fname;
    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    (out_fname, http_fname)
}

pub fn process_posters(x: String, count: u32) -> Vec<String> {
    let mut bad_image_vec = vec![];
    let dims = crate::mtv_image::get_image_dims(&x);
    if dims != (0, 0) {
        let img_path = &x;
        let img_size = crate::mtv_utils::get_file_size(&x);
        let name = crate::mtv_utils::split_poster_name(x.clone());
        let thumb_path = create_movie_thumbnail(x.clone());
        let file_thumb_path = thumb_path.0;
        let http_thumb_path = thumb_path.1;
        let img_id = create_md5(&x);
        let movimg = MovieImage {
            id: count,
            imgid: img_id,
            path: x.clone(),
            imgpath: img_path.to_string(),
            size: img_size.to_string(),
            name: name,
            thumbpath: file_thumb_path,
            idx: count,
            httpthumbpath: http_thumb_path,
        };
        println!("movimg: {:?}", movimg);

        let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
        let conn = Connection::open(db_path).expect("unable to open db file");
        conn.execute(
            "INSERT INTO images (ImgId, Path, ImgPath, Size, Name, ThumbPath, Idx, HttpThumbPath) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&movimg.imgid, &movimg.path, &movimg.imgpath, &movimg.size, &movimg.name, &movimg.thumbpath, &movimg.idx.to_string(), &movimg.httpthumbpath],
        )
        .expect("Unable to insert new tvshow info");
    } else {
        bad_image_vec.push(x.clone());
    }

    bad_image_vec
}





pub fn get_image_dims(x: &String) -> (u32, u32) {
    let dims_rs = image::image_dimensions(&x);
    let dims = match dims_rs {
        Ok(d) => d,
        Err(_) => (0, 0),
    };

    dims
}

// pub fn normalize_music_image(dims: (u32, u32)) -> (u32, u32) {
//     let largest: u32;

//     if dims.0 == dims.1 {
//         largest = dims.0;
//     } else if dims.0 > dims.1 {
//         largest = dims.0;
//     } else {
//         largest = dims.1;
//     }

//     let resizetup: (u32, u32);
//     if largest < 100 {
//         resizetup = (100, 100);
//     } else if largest < 200 {
//         resizetup = (200, 200);
//     } else if largest < 300 {
//         resizetup = (300, 300);
//     } else {
//         resizetup = (300, 300);
//     }

//     resizetup
// }
