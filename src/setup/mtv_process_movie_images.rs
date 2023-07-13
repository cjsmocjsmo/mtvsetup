use image::{self};
use rusqlite::Connection;
use std::env;
// use serde::{Deserialize, Serialize};

use crate::setup::mtv_misc::create_md5;
use crate::setup::mtv_types::MovieImage;

fn create_movie_thumbnail(x: String) -> (String, String) {
    let mtv_movies_thumbnail_path =
        env::var("MTV_MOVIES_THUMBNAIL_PATH").expect("$MTV_MOVIES_THUMBNAIL_PATH is not set");
    let server_addr = env::var("MTV_SERVER_ADDR").expect("$MTV_SERVER_ADDR is not set");
    let server_port = env::var("MTV_SERVER_PORT").expect("$MTV_SERVER_PORT is not set");

    let old_fname = crate::setup::mtv_split::split_poster_name(x.clone());
    // let out_fname = mtv_movies_thumbnail_path + &old_fname;
    let out_fname = mtv_movies_thumbnail_path + &old_fname;

    let http_fname = server_addr + ":" + &server_port + "/" + &old_fname;
    
    println!("x: {}", x);
    println!("old_fname{}", old_fname);
    println!("out_fname: {}", out_fname);

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    (out_fname, http_fname)
}

pub fn process_movie_posters(x: String, count: u32) -> Vec<String> {
    let mut bad_image_vec = vec![];

    

    let dims = crate::setup::mtv_image::get_image_dims(&x);
    if dims != (0, 0) {
        let img_path = &x;
        let img_size = crate::setup::mtv_misc::get_file_size(&x);
        let name = crate::setup::mtv_split::split_poster_name(x.clone());
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

        println!("{:#?}", movimg);

        let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
        let conn = Connection::open(db_path).expect("unable to open db file");
        conn.execute(
            "INSERT INTO images (imgid, path, imgpath, size, name, thumbpath, idx, httpthumbpath) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&movimg.imgid, &movimg.path, &movimg.imgpath, &movimg.size, &movimg.name, &movimg.thumbpath, &movimg.idx.to_string(), &movimg.httpthumbpath],
        )
        .expect("Unable to insert new tvshow info");
    } else {
        bad_image_vec.push(x.clone());

        //     println!("this is a bad image:\n\t {}", x.clone());
        // }

        // let bad_image_count = bad_image_vec.clone().len();

        // if bad_image_count != 0 {
        //     let mtv_movies_thumbnail_path =
        //         env::var("MTV_MOVIES_METADATA_PATH").expect("$MTV_MOVIES_METADATA_PATH is not set");

        //     let a = format!("{}/", mtv_movies_thumbnail_path.as_str());
        //     let b = format!("Bad_Movies_Images.json");
        //     let outpath = a + &b;
        //     fs::write(outpath, bad_image_vec.join("\n"))
        //         .expect("Failed to write named incorrectly json file");
        // }

        // (bad_image_count.to_string(), count.to_string())
    }

    bad_image_vec
}