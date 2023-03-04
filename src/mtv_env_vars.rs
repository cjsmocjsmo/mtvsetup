use std::env;
use std::fs::File;
use std::io::prelude::*;
// use yaml_rust::yaml;
use yaml_rust::YamlLoader;

pub fn get_docker_var() -> String {
    let docker_var_results = env::var("MTV_DOCKER_VAR");
    let docker_var = match docker_var_results {
        Ok(docker_var) => docker_var,
        Err(_error) => "docker var not set".to_string(),
    };

    docker_var
}

pub fn set_env_var(p1: String, p2: String) -> Result<(), Box<dyn std::error::Error>> {
    env::set_var(&p1, p2);
    let value = env::var(&p1);
    if value.is_err() {
        println!("Error: key not found");
    } else {
        println!("key is set to: {}", value.unwrap());
    }

    Ok(())
}

pub fn read_config() {
    let mut file = File::open("./src/config.yaml").expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    for d in docs {
        let music0 = "MTV_MEDIA_PATH".to_string();
        let music1 = d["MTV_MEDIA_PATH"].as_str().unwrap().to_string();
        set_env_var(music0, music1).unwrap();

        let music2 = "MTV_MUSIC_PATH".to_string();
        let music3 = d["MTV_MUSIC_PATH"].as_str().unwrap().to_string();
        set_env_var(music2, music3).unwrap();

        let music4 = "MTV_MUSIC_THUMBNAIL_PATH".to_string();
        let music5 = d["MTV_MUSIC_THUMBNAIL_PATH"].as_str().unwrap().to_string();
        set_env_var(music4, music5).unwrap();

        let music6 = "MTV_MUSIC_METADATA_PATH".to_string();
        let music7 = d["MTV_MUSIC_METADATA_PATH"].as_str().unwrap().to_string();
        set_env_var(music6, music7).unwrap();

        let music8 = "MTV_MOVIES_PATH".to_string();
        let music9 = d["MTV_MOVIES_PATH"].as_str().unwrap().to_string();
        set_env_var(music8, music9).unwrap();

        let music10 = "MTV_MOVIES_THUMBNAIL_PATH".to_string();
        let music11 = d["MTV_MOVIES_THUMBNAIL_PATH"].as_str().unwrap().to_string();
        set_env_var(music10, music11).unwrap();

        let music12 = "MTV_MOVIES_METADATA_PATH".to_string();
        let music13 = d["MTV_MOVIES_METADATA_PATH"].as_str().unwrap().to_string();
        set_env_var(music12, music13).unwrap();

        let offset1 = String::from("MTV_OFFSET");
        let offset2 = String::from("25");
        set_env_var(offset1, offset2).unwrap();


    }
}
