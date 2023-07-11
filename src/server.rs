// use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::{get, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use std::env;

// use crate::setup::mtv_types::MovieImage;
// use crate::setup::mtv_types::Movie;
// use crate::setup::mtv_types::TVShow;

#[get("/test")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/action")]
pub async fn action() -> impl Responder {
    let action = String::from("action");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&action]).expect("Unable to query db");
    // let mut exists = false;
    let mut result = Vec::new();
    while let Some(row) = rows.next().expect("Unable to get next row") {
        let movie = crate::setup::mtv_types::Movie {
            id: row.get(0).expect("Unable to get id"),
            name: row.get(1).expect("Unable to get name"),
            year: row.get(2).expect("Unable to get year"),
            posteraddr: row.get(3).expect("Unable to get posteraddr"),
            size: row.get(4).expect("Unable to get size"),
            path: row.get(5).expect("Unable to get path"),
            idx: row.get(6).expect("Unable to get idx"),
            movid: row.get(7).expect("Unable to get movid"),
            catagory: row.get(8).expect("Unable to get catagory"),
        };
        result.push(movie);
    }
    let result = serde_json::to_string(&result).unwrap();
    HttpResponse::Ok().json(result)
   
}

#[get("/arnold")]
pub async fn arnold() -> impl Responder {
    HttpResponse::Ok().body("arnold")
}

#[get("/brucewillis")]
pub async fn brucewillis() -> impl Responder {
    HttpResponse::Ok().body("brucewillis")
}

#[get("/cartoons")]
pub async fn cartoons() -> impl Responder {
    HttpResponse::Ok().body("cartoons")
}

#[get("/comedy")]
pub async fn comedy() -> impl Responder {
    HttpResponse::Ok().body("comedy")
}

#[get("/drama")]
pub async fn drama() -> impl Responder {
    HttpResponse::Ok().body("drama")
}

#[get("/documentary")]
pub async fn documentary() -> impl Responder {
    let documentary = String::from("documentary");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&documentary]).expect("Unable to query db");
    // let mut exists = false;
    let mut result = Vec::new();
    while let Some(row) = rows.next().expect("Unable to get next row") {
        let movie = crate::setup::mtv_types::Movie {
            id: row.get(0).expect("Unable to get id"),
            name: row.get(1).expect("Unable to get name"),
            year: row.get(2).expect("Unable to get year"),
            posteraddr: row.get(3).expect("Unable to get posteraddr"),
            size: row.get(4).expect("Unable to get size"),
            path: row.get(5).expect("Unable to get path"),
            idx: row.get(6).expect("Unable to get idx"),
            movid: row.get(7).expect("Unable to get movid"),
            catagory: row.get(8).expect("Unable to get catagory"),
        };
        result.push(movie);
    }
    let result = serde_json::to_string(&result).unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/fantasy")] 
pub async fn fantasy() -> impl Responder {
    HttpResponse::Ok().body("fantasy")
}

#[get("/godzilla")]
pub async fn godzilla() -> impl Responder {
    HttpResponse::Ok().body("godzilla")
}

#[get("/harrypotter")]
pub async fn harrypotter() -> impl Responder {
    HttpResponse::Ok().body("harrypotter")
}

#[get("/indianajones")]
pub async fn indianajones() -> impl Responder {
    HttpResponse::Ok().body("indianajones")
}

#[get("/jamesbond")]
pub async fn jamesbond() -> impl Responder {
    HttpResponse::Ok().body("jamesbond")
}

#[get("/johnwayne")]
pub async fn johnwayne() -> impl Responder {
    HttpResponse::Ok().body("johnwayne")
}

#[get("/JohnWick")]
pub async fn johnwick() -> impl Responder {
    HttpResponse::Ok().body("JohnWick")
}

#[get("/jurassicpark")]
pub async fn jurassicpark() -> impl Responder {
    HttpResponse::Ok().body("jurassicpark")
}

#[get("/kingsmen")]
pub async fn kingsmen() -> impl Responder {
    HttpResponse::Ok().body("kingsmen")
}

#[get("/meninblack")]
pub async fn meninblack() -> impl Responder {
    HttpResponse::Ok().body("meninblack")
}

#[get("/misc")]
pub async fn misc() -> impl Responder {
    HttpResponse::Ok().body("misc")
}

#[get("/nicolascage")]
pub async fn nicolascage() -> impl Responder {
    HttpResponse::Ok().body("nicolascage")
}

#[get("/pirates")]
pub async fn pirates() -> impl Responder {
    HttpResponse::Ok().body("pirates")
}

#[get("/riddick")]
pub async fn riddick() -> impl Responder {
    HttpResponse::Ok().body("riddick")
}

#[get("/starwars")]
pub async fn starwars() -> impl Responder {
    HttpResponse::Ok().body("starwars")
}

#[get("/startrek")]
pub async fn startrek() -> impl Responder {
    HttpResponse::Ok().body("startrek")
}

#[get("/superheroes")]
pub async fn superheroes() -> impl Responder {
    HttpResponse::Ok().body("superheroes")
}

#[get("/scifi")]
pub async fn scifi() -> impl Responder {
    HttpResponse::Ok().body("scifi")
}

#[get("/tomcruize")]
pub async fn tomcruize() -> impl Responder {
    HttpResponse::Ok().body("tomcruize")
}

#[get("/transformers")]
pub async fn transformers() -> impl Responder {
    HttpResponse::Ok().body("transformers")
}

#[get("/tremors")]
pub async fn tremors() -> impl Responder {
    HttpResponse::Ok().body("tremors")
}

#[get("/therock")]
pub async fn therock() -> impl Responder {
    HttpResponse::Ok().body("therock")
}

#[get("/xmen")]
pub async fn xmen() -> impl Responder {
    HttpResponse::Ok().body("xmen")
}