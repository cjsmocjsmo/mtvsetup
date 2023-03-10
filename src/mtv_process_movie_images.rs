use image::{self};
use std::env;
use std::fs;

fn create_movie_thumbnail(x: String) {
    let mtv_music_metadata_path =
        env::var("MTV_MOVIES_THUMBNAIL_PATH").expect("$MTV_MOVIES_THUMBNAIL_PATH is not set");
    let old_fname = crate::mtv_split::split_poster_name(x.clone());
    let out_fname = mtv_music_metadata_path + "/" + &old_fname;

    // println!("this is out_fname:\n\t{}", out_fname);

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail.save(out_fname).expect("Saving image failed");
}

pub fn process_movie_posters(xx: Vec<String>) {
    let mut index = 0;
    let mut bad_image_vec = vec![];

    for x in xx {
        index = index + 1;

        let dims = crate::mtv_image::get_image_dims(&x);
        if dims != (0, 0) {
            let img_path = &x;
            let img_size = crate::mtv_misc::get_file_size(&x);
            let name = crate::mtv_split::split_poster_name(x.clone());
            // println!("\nthis is img_path:\n\t{}", img_path);
            // println!("this is dims:\n\t{:?}", dims);
            // println!("this is img_size:\n\t{}", img_size);
            // println!("this is name:\n\t{}", name);

            create_movie_thumbnail(x);
        } else {
            bad_image_vec.push(x.clone());

            println!("this is a bad image:\n\t {}", x.clone());
        }
    }
    let bad_image_count = bad_image_vec.clone().len();

    if bad_image_count != 0 {
        let mtv_music_metadata_path =
            env::var("MTV_MOVIES_METADATA_PATH").expect("$MTV_MOVIES_METADATA_PATH is not set");

        let a = format!("{}/", mtv_music_metadata_path.as_str());
        let b = format!("Bad_Movies_Images.json");
        let outpath = a + &b;
        fs::write(outpath, bad_image_vec.join("\n"))
            .expect("Failed to write named incorrectly json file");
    }
    println!("\n\nThere are {} bad movies images", bad_image_count);
    println!("There are {} movie images total. \n\n", index);
}
