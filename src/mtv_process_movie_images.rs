use image::{self};
// use json::object;
use std::env;
// use std::fs;

use crate::{mtv_types::MovieImage, mtv_misc::create_md5};

fn create_movie_thumbnail(x: String) -> String {
    let mtv_movie_metadata_path =
        env::var("MTV_MOVIES_THUMBNAIL_PATH").expect("$MTV_MOVIES_THUMBNAIL_PATH is not set");
    let old_fname = crate::mtv_split::split_poster_name(x.clone());
    let out_fname = mtv_movie_metadata_path + "/" + &old_fname;

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    out_fname
}

pub fn process_movie_posters(x: String, count: u32) -> String {
    // let mut bad_image_vec = vec![];

    let dims = crate::mtv_image::get_image_dims(&x);
    if dims != (0, 0) {
        let img_path = &x;
        let img_size = crate::mtv_misc::get_file_size(&x);
        let name = crate::mtv_split::split_poster_name(x.clone());
        let thumb_path = create_movie_thumbnail(x.clone());
        let img_id = create_md5(&x);

        let movimg = MovieImage {
            id: count,
            imgid: img_id,
            path: x.clone(),
            imgpath: img_path.to_string(),
            size: img_size.to_string(),
            name: name,
            thumbpath: thumb_path,
            idx: count
        };

        println!("{:#?}", movimg);

        // let mov_img_obj = object! {
        //     path: img_path.to_string(),
        //     size: img_size.to_string(),
        //     name: name,
        //     thumbpath: thumb_path,
        // };

        // let mov_img_info = json::stringify(mov_img_obj.dump());

        // let mtv_movie_metadata_path =
        //     env::var("MTV_MOVIES_METADATA_PATH").expect("$MTV_MOVIES_METADATA_PATH is not set");

        // let a = format!("{}/", mtv_movie_metadata_path.as_str());
        // let b = format!("Movie_Image_{}_Info.json", count.to_string());
        // let outpath = a + &b;
        // fs::write(outpath, mov_img_info).expect("Failed to write named incorrectly json file");
    // } else {
    //     bad_image_vec.push(x.clone());

    //     println!("this is a bad image:\n\t {}", x.clone());
    // }

    // let bad_image_count = bad_image_vec.clone().len();

    // if bad_image_count != 0 {
    //     let mtv_movie_metadata_path =
    //         env::var("MTV_MOVIES_METADATA_PATH").expect("$MTV_MOVIES_METADATA_PATH is not set");

    //     let a = format!("{}/", mtv_movie_metadata_path.as_str());
    //     let b = format!("Bad_Movies_Images.json");
    //     let outpath = a + &b;
    //     fs::write(outpath, bad_image_vec.join("\n"))
    //         .expect("Failed to write named incorrectly json file");
    // }

    // (bad_image_count.to_string(), count.to_string())
    
    }
    "ruck".to_string()
}
