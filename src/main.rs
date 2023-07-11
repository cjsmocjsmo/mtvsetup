// sudo apt-get install -y  build-essential \
//     jq \
//     libsodium-dev \
//     libsnappy-dev \
//     libssl-dev \
//     pkg-config \
//     rocksdb-tools \
//     librocksdb-dev \
//     librocksdb7.8 \
//     librust-clang-sys+libloading-dev \
//     librust-clang-sys-dev \
//     libprotobuf-dev \
//     libprotoc-dev \
//     protobuf-compiler

// use rusqlite::Connection;

// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
// use std::env;
// use log::{error, info, debug};
// use actix_files as fs;

pub mod setup;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    let setup = crate::setup::setup();

    if setup {
        HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);

            App::new().wrap(cors)
            // .service(crate::func::hello)
            // .service(crate::func::insert_review)

            // .route(
            //     "/hey",
            //     web::get().to(crate::func::server_functions::manual_hello),
            // )

            // .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
        })
        .bind(("192.168.0.91", 8080))?
        .run()
        .await?;
    }
    Ok(())
}
