pub fn process_mp3s() {
    let mp3svec = crate::mtv_walk_dirs::walk_music_dir_mp3();

    let mut index = 0;
    for mp3 in mp3svec {
        index = index + 1;

        let id = crate::mtv_misc::get_md5(&mp3);
        let voodoo: &String = &"None".to_string();
        let tags = crate::mtv_mp3_info::get_tag_info(&mp3);
        let artist = tags.0;
        let album = tags.1;
        let song = tags.2;
        let genre = tags.3;
        let base_dir = crate::mtv_split::split_base_dir(&mp3);
        let filename_results = crate::mtv_split::split_filename(&mp3);
        let music_artist_results = crate::mtv_split::music_split_artist(&base_dir);
        let music_album_results = crate::mtv_split::music_split_album(&base_dir);
        let duration_results = crate::mtv_mp3_info::get_duration(&mp3);
        let artc = crate::mtv_mp3_info::check_artist(&music_artist_results, &artist);
        let albc = crate::mtv_mp3_info::check_album(&music_album_results, &album);
        let sc = crate::mtv_mp3_info::check_song(&filename_results, &song);
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
            artc,
            albc,
            sc,
            fullpath.to_string(),
            extension,
            idx,
            fsize_results,

        );

        // if artc == true && albc == true && sc == true {
        //     println!("\n they all match:\n {}", &mp3);

        //     let mp3_info = object! {
        //         mp3id: mtvserver::get_md5(&mp3),
        //         fullpath: &*mp3,
        //         basedir: &*base_dir,
        //         filename: filename_results,
        //         ext: mtvserver::split_ext(&mp3),
        //         imgurl: &**voodoo,
        //         mp3_url: &**voodoo,
        //         tag_artist: &*tags.0,
        //         tag_album: &*tags.1,
        //         tag_title: &*tags.2,
        //         tag_genre: &*tags.3,
        //         idx: index,
        //         fsize: mtvserver::get_file_size(&mp3),
        //         filename_artist: &*music_artist_results,
        //         filename_album: &*music_album_results,
        //         duration: duration_results,
        //     };

        //     let mfo: String = json::stringify(mp3_info.dump());

        //     let mtv_music_metadata_path =
        //         env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

        //     let a = format!("{}/", mtv_music_metadata_path.as_str());
        //     let b = format!("Music_File_Meta_{}.json", index);
        //     let outpath = a + &b;
        //     std::fs::write(outpath, mfo.clone()).unwrap();

        //     println!("\n\n\n mp3info {}", mfo.clone());
        // } else {
        //     // println!("{:?}", mp3.clone());
        //     named_incorrectly_vec.push(mp3.clone());
        
    }
    // println!(
    //     "there are {} mp3s named incorrectly",
    //     named_incorrectly_vec.len()
    // );

    // for name in named_incorrectly_vec {
    //     println!("nameed incorrectly with tags {}", name);
    // }
    // println!("There are {} mp3s", index.to_string());
}