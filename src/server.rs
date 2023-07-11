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
    let searcht = String::from("Action");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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
    let searcht = String::from("Arnold");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/brucewillis")]
pub async fn brucewillis() -> impl Responder {
    let searcht = String::from("BruceWillis");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/cartoons")]
pub async fn cartoons() -> impl Responder {
    let searcht = String::from("Cartoons");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/comedy")]
pub async fn comedy() -> impl Responder {
    let searcht = String::from("Comedy");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/drama")]
pub async fn drama() -> impl Responder {
    let searcht = String::from("Drama");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/documentary")]
pub async fn documentary() -> impl Responder {
    let searcht = String::from("Documentary");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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
    let searcht = String::from("Fantasy");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/godzilla")]
pub async fn godzilla() -> impl Responder {
    let searcht = String::from("Godzilla");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/harrypotter")]
pub async fn harrypotter() -> impl Responder {
    let searcht = String::from("HarryPotter");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/indianajones")]
pub async fn indianajones() -> impl Responder {
    let searcht = String::from("IndianaJones");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/jamesbond")]
pub async fn jamesbond() -> impl Responder {
    let searcht = String::from("JamesBond");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/johnwayne")]
pub async fn johnwayne() -> impl Responder {
    let searcht = String::from("JohnWayne");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/johnwick")]
pub async fn johnwick() -> impl Responder {
    let searcht = String::from("JohnWick");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/jurassicpark")]
pub async fn jurassicpark() -> impl Responder {
    let searcht = String::from("JurassicPark");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/kingsmen")]
pub async fn kingsmen() -> impl Responder {
    let searcht = String::from("KingsMen");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/meninblack")]
pub async fn meninblack() -> impl Responder {
    let searcht = String::from("MenInBlack");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/misc")]
pub async fn misc() -> impl Responder {
    let searcht = String::from("Misc");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/nicolascage")]
pub async fn nicolascage() -> impl Responder {
    let searcht = String::from("NicolasCage");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/pirates")]
pub async fn pirates() -> impl Responder {
    let searcht = String::from("Pirates");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/riddick")]
pub async fn riddick() -> impl Responder {
    let searcht = String::from("Riddick");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/starwars")]
pub async fn starwars() -> impl Responder {
    let searcht = String::from("StarWars");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/startrek")]
pub async fn startrek() -> impl Responder {
    let searcht = String::from("StarTrek");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/superheroes")]
pub async fn superheroes() -> impl Responder {
    let searcht = String::from("SuperHeroes");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/scifi")]
pub async fn scifi() -> impl Responder {
    let searcht = String::from("SciFi");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/tomcruize")]
pub async fn tomcruize() -> impl Responder {
    let searcht = String::from("TomCruize");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/transformers")]
pub async fn transformers() -> impl Responder {
    let searcht = String::from("Transformers");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/tremors")]
pub async fn tremors() -> impl Responder {
    let searcht = String::from("Tremors");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/therock")]
pub async fn therock() -> impl Responder {
    let searcht = String::from("TheRock");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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

#[get("/xmen")]
pub async fn xmen() -> impl Responder {
    let searcht = String::from("XMen");
    let db_path = env::var("MTV_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE catagory = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&searcht]).expect("Unable to query db");
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