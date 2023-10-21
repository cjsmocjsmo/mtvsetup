use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use crate::setup;
use std::env;

#[get("/test")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/stats")]
pub async fn get_stats() -> impl Responder {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM stats")
        .unwrap();
    let mut rows = stmt.query([]).expect("Unable to query db");
    let mut result = Vec::new();
    while let Some(row) = rows.next().expect("Unable to get next row") {
        let stat = crate::setup::mtv_types::Stats {
            id: row.get(0).expect("Unable to get id"),
            moviecount: row.get(1).expect("Unable to get moviecount"),
            tvshowcount: row.get(2).expect("Unable to get tvshowcount"),
            postercount: row.get(3).expect("Unable to get postercount"),
            size: row.get(4).expect("Unable to get fsize"),
        };
        println!("id: {}, moviecount: {}, tvshowcount: {}, postercount: {}, size: {}", stat.id, stat.moviecount, stat.tvshowcount, stat.postercount, stat.size);
        result.push(stat);
    }
    let results = serde_json::to_string(&result).unwrap();

    HttpResponse::Ok().body(results)
}

#[get("/setupcheck/{status}")]
pub async fn run_setup_check(path: web::Path<String>) -> impl Responder {
    let _status = path.into_inner();
    let file_exists = setup::mtv_tables::db_file_exists().to_string();
    let dir_exists = setup::mtv_image::thumbnail_dir_exists().to_string();
    let mtv_file = setup::mtv_utils::mtvsetup_file_check().to_string();
    let mut resp = "false".to_string();
    if dir_exists == "true" && file_exists == "true" && mtv_file == "true"{
        resp = "true".to_string();
    };
    HttpResponse::Ok().body(resp)
}

#[get("/setup/{status}")]
pub async fn run_setup(path: web::Path<String>) -> impl Responder {
    let _fuck = path.into_inner();
    let setup = setup::setup().to_string();
    let file_exists = setup::mtv_tables::db_file_exists().to_string();
    let dir_exists = setup::mtv_image::thumbnail_dir_exists().to_string();
    let mtv_file = setup::mtv_utils::mtvsetup_file_check().to_string();
    let mut resp = "false".to_string();
    if setup == "true" && dir_exists == "true" && file_exists == "true" && mtv_file == "true"{
        resp = "true".to_string();
    };
    HttpResponse::Ok().body(resp)
}