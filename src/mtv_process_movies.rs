// use json::object;
// use std::env;
use std::path::Path;

fn get_poster_addr(x: String) -> String {
    let no_ext_name_res = x.split(".");
    let mut no_ext_name_vec = vec![];

    for n in no_ext_name_res {
        no_ext_name_vec.push(n);
    }

    let new_jpg_name = no_ext_name_vec[0].to_owned() + ".jpg";
    let new_jpg_name_split = new_jpg_name.split("Movies");
    let mut new_jpg_name_split_vec = vec![];

    for jpg in new_jpg_name_split {
        new_jpg_name_split_vec.push(jpg);
    }
    let p1 = new_jpg_name_split_vec[0];
    let p2 = new_jpg_name_split_vec[1];
    let p2_split = p2.split("/");
    let mut p2_split_vec = vec![];
    for p in p2_split {
        p2_split_vec.push(p);
    }

    let poster_addr = p1.to_string() + &"Posters2/".to_string() + p2_split_vec[2];

    poster_addr
}

pub fn process_movies(x: String, count: u32) -> String {
    // let movies_vec = crate::mtv_walk_dirs::walk_movies_dir(usb1);
    // let moviez = movies_vec[0].clone();

    

        let mov_name = crate::mtv_split::split_movie_name(x.clone());
        let mov_year = crate::mtv_split::split_movie_year(x.clone());
        let mov_poster_addr = get_poster_addr(x.clone());
        let mov_size = crate::mtv_misc::get_file_size(&x);
        let mov_file_exists = Path::new(&mov_poster_addr).exists();
        let mov_id = crate::mtv_misc::create_md5(&x);

        let mojo = crate::mtv_types::Movie {
            name: mov_name,
            year: mov_year,
            poster_addr: mov_poster_addr,
            size: mov_size.to_string(),
            exists: mov_file_exists,
            path: x,
            index: count.to_string(),
            movid: mov_id,
        };

        // println!("{:#?}", mojo);

    

    "fuck".to_string()
}


