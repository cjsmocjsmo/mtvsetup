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
    }
}