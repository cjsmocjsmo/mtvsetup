use std::env;

// Production settings
pub fn set_env_vars() {
    let mov_path = env::var("MTV_MOVIES_PATH");
    if mov_path.is_err() {
        env::set_var("MTV_MOVES_PATH", "/home/pimedia/PINAS/bazmnt/Movies/");
    }
    println!("MTV_MOVIES_PATH: {:?}", env::var("MTV_MOVIES_PATH"));

    let tv_path = env::var("MTV_TV_PATH");
    if tv_path.is_err() {
        env::set_var("MTV_TV_PATH", "/home/pimedia/PINAS/bazmnt/TVShows/");
    }

    let thumb_path = env::var("MTV_MOVIES_THUMBNAIL_PATH");
    if thumb_path.is_err() {
        env::set_var("MTV_MOVIES_THUMBNAIL_PATH", "/usr/share/mtvsetup/mtvsetup/thumbnails/");
    }

    let static_path = env::var("MTV_STATIC_PATH");
    if static_path.is_err() {
        env::set_var("MTV_STATIC_PATH", "/usr/share/mtvsetup/mtvsetup/static");
    }

    let raw_addr = env::var("MTV_RAW_ADDR");
    if raw_addr.is_err() {
        env::set_var("MTV_RAW_ADDR", "192.168.0.104");
    }

    let server_addr = env::var("MTV_SERVER_ADDR");
    if server_addr.is_err() {
        env::set_var("MTV_SERVER_ADDR", "http://192.168.0.104");
    }

    let server_port = env::var("MTV_SERVER_PORT");
    if server_port.is_err() {
        env::set_var("MTV_SERVER_PORT", "8080");
    }

    let docker_var = env::var("MTV_DOCKER_VAR");
    if docker_var.is_err() {
        env::set_var("MTV_DOCKER_VAR", "NONE");
    }

    let offset = env::var("MTV_OFFSET");
    if offset.is_err() {
        env::set_var("MTV_OFFSET", "25");
    }

    let db_path = env::var("MTV_DB_PATH");
    if db_path.is_err() {
        env::set_var("MTV_DB_PATH", "/usr/share/mtvsetup/mtvsetup/db/mtv.db");
    }
    // let db_check_file_path = env::var("MTV_DB_CHECK_FILE_PATH");
    // if db_check_file_path.is_err() {
    //     env::set_var("MTV_DB_CHECK_FILE_PATH", "/usr/share/mtvsetup/mtvsetup/db/db_check_file.txt");
    // }

    let db_path = env::var("MTV_FILE_PATH");
    if db_path.is_err() {
        env::set_var("MTV_FILE_PATH", "/usr/share/mtvsetup/mtvsetup/setup.mtv");
    }

}

