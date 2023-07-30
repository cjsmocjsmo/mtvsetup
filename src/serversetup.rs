use actix_web::{get, web, HttpResponse, Responder};
use crate::setup;

#[get("/setupcheck/{status}")]
pub async fn run_setup_check(path: web::Path<String>) -> impl Responder {
    let _status = path.into_inner();
    let file_exists = setup::mtv_tables::db_file_exists().to_string();
    let dir_exists = setup::mtv_image::thumbnail_dir_exists().to_string();
    let mut resp = "false".to_string();
    if dir_exists == "true" && file_exists == "true" {
        resp = "true".to_string();
    };
    HttpResponse::Ok().body(resp)
}

#[get("/setup/{status}")]
pub async fn run_setup(path: web::Path<String>) -> impl Responder {
    let _fuck = path.into_inner();
    let stp = setup::setup();
    let setup = stp.to_string();
    HttpResponse::Ok().body(setup)
}