use filesize::PathExt;
use md5::{Digest, Md5};
use std::path::Path;

// pub fn split_filename(x: &String) -> String {
//     let filesplit = x.split("/");
//     let mut filenamevec = vec![];
//     for file in filesplit {
//         filenamevec.push(file);
//     }

//     let count = &filenamevec.len() - 1;
//     filenamevec.drain(0..count);
//     let mut finalvec = "";
//     for f in filenamevec {
//         finalvec = f;
//     }

//     let fname = finalvec.split(".");
//     let mut svec = vec![];
//     for f in fname {
//         svec.push(f);
//     }
//     svec.pop();

//     let filename = svec.get(0).unwrap();

//     filename.to_string()
// }

pub fn split_movie_name(x: String) -> String {
    let filesplit = x.split("/");

    let mut filenamevec: Vec<String> = vec![];
    for file in filesplit {
        filenamevec.push(file.to_string());
    }
    let raw_fname = filenamevec.pop().unwrap();

    let fsplit = raw_fname.split(" (");
    let mut fsplit_vec = vec![];
    for f in fsplit {
        fsplit_vec.push(f);
    }

    fsplit_vec[0].to_string()
}

pub fn split_movie_year(x: String) -> String {
    let filesplit = x.split("/");

    let mut filenamevec: Vec<String> = vec![];
    for file in filesplit {
        filenamevec.push(file.to_string());
    }
    let raw_fname = filenamevec.pop().unwrap();

    let fsplit = raw_fname.split(" (");
    let mut fsplit_vec = vec![];
    for f in fsplit {
        fsplit_vec.push(f);
    }

    let fsplit2 = fsplit_vec[1].split(")");
    let mut fsplit_vec2 = vec![];
    for f2 in fsplit2 {
        fsplit_vec2.push(f2);
    }

    fsplit_vec2[0].to_string()
}

pub fn split_poster_name(x: String) -> String{
    let filesplit = x.split("/");

    let mut filenamevec: Vec<String> = vec![];
    for file in filesplit {
        filenamevec.push(file.to_string());
    }
    let fname = filenamevec.pop().unwrap();

    fname
}

pub fn create_md5(astring: &String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(&astring);
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

pub fn get_file_size(x: &String) -> u64 {
    let path = Path::new(&x);

    path.size_on_disk().unwrap()
}

// pub fn media_total_size(addr: String) -> String {
//     let total_size = WalkDir::new(addr)
//         .min_depth(1)
//         .max_depth(5)
//         .into_iter()
//         .filter_map(|entry| entry.ok())
//         .filter_map(|entry| entry.metadata().ok())
//         .filter(|metadata| metadata.is_file())
//         .fold(0, |acc, m| acc + m.len());

//     let btos = total_size.to_string();
//     let result = Byte::from_str(btos).unwrap();
//     let size = result.get_appropriate_unit(true).to_string();

//     size
// }
