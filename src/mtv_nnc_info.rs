use json::{object, JsonValue};

struct CompParts {
    dir_artist: String,
    dir_album: String,
    file_artist: String,
    file_album: String,
    file_song: String,
    tag_artist: String,
    tag_album: String,
    tag_song: String
}

fn get_dir_artist(j: JsonValue) -> String {
    let mut jstr = j.to_string();
    let mut boo = jstr.split("/");
    let mut newboovec = vec![];
    for b in boo {
        newboovec.push(b);
    }
    newboovec.pop();
    newboovec.pop();
    let foobar = newboovec.pop().unwrap();
    let foo = foobar.split("_");
    let mut bar = vec![];
    for f in foo {
        bar.push(f);
    }
    let artist = bar.join(" ");

    artist
}

fn get_dir_album(j: JsonValue) -> String {
    let mut jstr = j.to_string();
    let mut boo = jstr.split("/");
    let mut newboovec = vec![];
    for b in boo {
        newboovec.push(b);
    }
    newboovec.pop();
    let foobar = newboovec.pop().unwrap();
    let foo = foobar.split("_");
    let mut bar = vec![];
    for f in foo {
        bar.push(f);
    }
    let album = bar.join(" ");

    album
}

fn get_fn_artist(x: &String) -> (String, String, String) {
    let art_split1 = x.split("_-_");
    let mut art_split1_vec = vec![];
    for art in art_split1 {
        art_split1_vec.push(art);
    };

    let artist_res = art_split1_vec[1];
    let album = art_split1_vec[2];
    let song = art_split1_vec[3];

    let art = artist_res.split("_");
    let mut art_vec = vec![];
    for a in art {
        art_vec.push(a);
    };
    let artist = art_vec.join(" ");





    // let song = art_split1_vec.pop().unwrap();
    // let albuma = art_split1_vec.pop().unwrap();
    // let alb = albuma.split("_");
    // let mut alb_vec = vec![];
    // for a in alb {
    //     alb_vec.push(a);
    // };

    // let album = alb_vec.join(" ");

    // let artist = art_split1_vec.pop().unwrap();

    let meta = (artist.to_string(), album.to_string(), song.to_string());

    meta




}

pub fn gather_media_info(xx: Vec<JsonValue>) {
    for x in xx {
        let dir_album = get_dir_album(x.clone());
        let dir_artist = get_dir_artist(x.clone());

        let foobar = &x["filename"].to_string();


        let meta = get_fn_artist(foobar);
        let file_artist = meta.0;
        let file_album = meta.1;
        let file_song = meta.2;
        
        
        let tags = crate::mtv_mp3_info::get_tag_info(foobar);
        let tag_artist = tags.0;
        let tag_album = tags.1;
        let tag_song = tags.2;

        if dir_artist != file_artist || file_artist != tag_artist || dir_artist != tag_artist {
            let zoo = format!("Artist Dont Match:\n\t dir: {}, file: {}, tag: {}", dir_artist, file_artist, tag_artist);
            println!("{}", zoo);
        } else {
            let zoo = format!("ArtistMatch:\n\t dir: {}, file: {}, tag: {}", dir_artist, file_artist, tag_artist);
            println!("{}", zoo);
        }

        if dir_album != file_album || file_album != tag_album || dir_album != tag_album {
            let moo = format!("Albums Dont Match:\n\t dir: {}, file: {}, tag: {}", dir_album, file_album, tag_album);
            println!("{}", moo);
        }

        
        // println!("this is tag album {}", tags.1);
    }

}