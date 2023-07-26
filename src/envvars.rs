use std::env;

pub fn set_env_vars() {

    let usb1 = env::var("MTV_USB1");
    if usb1.is_err() {
        env::set_var("MTV_USB1", "/media/pi/USB2/media");
    }
    let usb2 = env::var("MTV_USB2");
    if usb2.is_err() {
        env::set_var("MTV_USB2", "/media/pi/USB3/media");
    }
    let usb3 = env::var("MTV_USB3");
    if usb3.is_err() {
        env::set_var("MTV_USB3", "/media/pi/USB3/media");
    }
    let usb4 = env::var("MTV_USB4");
    if usb4.is_err() {
        env::set_var("MTV_USB4", "None");
    }
    let thumb_path = env::var("MTV_MOVIES_THUMBNAIL_PATH");
    if thumb_path.is_err() {
        env::set_var("MTV_MOVIES_THUMBNAIL_PATH", "/usr/share/mtvsetup/mtvsetup/thumbnails/");
    }
    let static_path = env::var("MTV_STATIC_PATH");
    if static_path.is_err() {
        env::set_var("MTV_STATIC_PATH", "/usr/share/static");
    }
    let raw_addr = env::var("MTV_RAW_ADDR");
    if raw_addr.is_err() {
        env::set_var("MTV_RAW_ADDR", "192.168.0.26");
    }
    let server_addr = env::var("MTV_SERVER_ADDR");
    if server_addr.is_err() {
        env::set_var("MTV_SERVER_ADDR", "http://http://192.168.0.26");
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
        env::set_var("MTV_DB_PATH", "/usr/share/mtvsetup/mtvsetup/mtv.db");
    }



}