use std::path::Path;
use std::env;

pub fn split_ext(astring: &String) -> String {
    let path = Path::new(&astring);
    let boo_results = path.extension();
    let boo = match boo_results {
        Some(b) => b.to_string_lossy().to_string(),
        None => "split_ext did not work".to_string(),
    };

    let ext = ".".to_string() + boo.as_str();

    ext
}

pub fn split_base_dir(astring: &String) -> String {
    let mysplit = astring.split("/");
    let mut myvec = vec![];

    for my in mysplit {
        myvec.push(my);
    }

    let path = env::var("MTV_MUSIC_PATH").unwrap();
    let envsplit = path.split("/");

    let mut envvec = vec![];

    for env in envsplit {
        envvec.push(env);
    }

    let count = envvec.len() - 1;
    myvec.drain(0..count);
    myvec.pop();

    let base_dir = myvec.join("/");

    base_dir
}

pub fn image_split_artist(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let mut fin = vec![];
    for f in filenamevec {
        fin.push(f);
    }

    String::from(fin[1])
}

pub fn music_split_artist(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let mut fin = vec![];
    for f in filenamevec {
        fin.push(f);
    }
    String::from(fin[2])
}

pub fn image_split_album(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let album_result = filenamevec.last();
    let album = match album_result {
        Some(album) => album.to_string(),
        None => "None".to_string(),
    };

    album.to_string()
}

pub fn music_split_album(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let count = &filenamevec.len() - 2;
    filenamevec.drain(0..count);
    let mut album = "";
    for f in filenamevec {
        album = f;
    }

    String::from(album)
}

pub fn split_filename(x: &String) -> String {
    let filesplit = x.split("/");
    let mut filenamevec = vec![];
    for file in filesplit {
        filenamevec.push(file);
    }

    let count = &filenamevec.len() - 1;
    filenamevec.drain(0..count);
    let mut finalvec = "";
    for f in filenamevec {
        finalvec = f;
    }

    let fname = finalvec.split(".");
    let mut svec = vec![];
    for f in fname {
        svec.push(f);
    }
    svec.pop();

    let filename = svec.get(0).unwrap();

    filename.to_string()
}