use json::object;
use std::env;

pub fn process_mp3s() -> String {
    let mp3svec = crate::mtv_walk_dirs::walk_music_dir_music();

    let mut index = 0;

    let mut page = 1;

    let mut page_count = 0;

    let ofs = env::var("MTV_OFFSET").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    let mut not_named_correctly = vec![];

    for mp3 in mp3svec {
        index = index + 1;
        if page_count < offset {
            page_count = page_count + 1;
            page = page;
        } else {
            page_count = 1;
            page = page + 1;
        }

        let tags = crate::mtv_mp3_info::get_tag_info(&mp3);
        let artist = tags.0;
        let album = tags.1;
        let song = tags.2;
        let genre = tags.3;

        let test_results = crate::mtv_nnc_info::test_media_sameness(
            mp3.clone(),
            artist.clone(),
            album.clone(),
            song.clone(),
        );
        if test_results {
            let id = crate::mtv_misc::get_md5(&mp3);
            let voodoo: &String = &"None".to_string();
            let base_dir = crate::mtv_split::split_base_dir(&mp3);
            let filename_results = crate::mtv_split::split_filename(&mp3);
            let music_artist_results = crate::mtv_split::music_split_artist(&base_dir);
            let music_album_results = crate::mtv_split::music_split_album(&base_dir);
            let duration_results = crate::mtv_mp3_info::get_duration(&mp3);

            let fullpath = &mp3.to_string();
            let extension = crate::mtv_split::split_ext(&mp3);
            let idx = index.to_string();
            let fsize_results = crate::mtv_misc::get_file_size(&mp3).to_string();

            crate::mtv_mp3_info::write_music_json_to_file(
                id,
                voodoo.to_string(),
                artist,
                album,
                song,
                genre,
                base_dir,
                filename_results,
                music_artist_results,
                music_album_results,
                duration_results,
                fullpath.to_string(),
                extension,
                idx,
                page.to_string(),
                fsize_results,
            );
        } else {
            println!("\n\tThe file parts and tags do not match:\n\t\t{}", &mp3);
            let not_named_correctly_entry = object! { filename:&*mp3 };
            not_named_correctly.push(not_named_correctly_entry);
        };
    }

    let not_named_correctly_count = not_named_correctly.len();

    let niv = json::stringify(not_named_correctly.clone());

    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

    let a = format!("{}/", mtv_music_metadata_path.as_str());
    let b = format!("Named_Incorrectly.json");
    let outpath = a + &b;

    std::fs::write(outpath, niv).unwrap();

    println!(
        "There are {} music files named incorrectly",
        not_named_correctly_count
    );

    index.to_string()
    // println!("There are {} music files processed", &index);
}
