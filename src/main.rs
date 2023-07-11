// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
// use serde::{Deserialize, Serialize};
// use log::{error, info, debug};

pub mod server;
pub mod setup;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    let setup = crate::setup::setup();
    let thumb_path =
        env::var("MTV_MOVIES_THUMBNAIL_PATH").expect("MTV_MOVIES_THUMBNAIL_PATH not set");
    if setup {
        HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .service(server::hello)
                .service(server::action)
                .service(server::arnold)
                .service(server::brucewillis)
                .service(server::cartoons)
                .service(server::comedy)
                .service(server::drama)
                .service(server::documentary)
                .service(server::fantasy)
                .service(server::godzilla)
                .service(server::harrypotter)
                .service(server::indianajones)
                .service(server::jamesbond)
                .service(server::johnwayne)
                .service(server::johnwick)
                .service(server::jurassicpark)
                .service(server::kingsmen)
                .service(server::meninblack)
                .service(server::misc)
                .service(server::nicolascage)
                .service(server::pirates)
                .service(server::riddick)
                .service(server::starwars)
                .service(server::startrek)
                .service(server::superheroes)
                .service(server::scifi)
                .service(server::tomcruize)
                .service(server::transformers)
                .service(server::tremors)
                .service(server::therock)
                .service(server::xmen)
                // .service(crate::func::insert_review)
                // .route(
                //     "/hey",
                //     web::get().to(crate::func::server_functions::manual_hello),
                // )
                .service(fs::Files::new("/thumbnails", thumb_path.clone()).show_files_listing())
        })
        .bind(("192.168.0.26", 8080))?
        .run()
        .await?;
    }
    Ok(())
}
