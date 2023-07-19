// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
// use serde::{Deserialize, Serialize};
// use log::{error, info, debug};

pub mod servermov;
pub mod servertvs;
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
                .service(servermov::hello)
                .service(servermov::action)
                .service(servermov::arnold)
                .service(servermov::brucelee)
                .service(servermov::brucewillis)
                .service(servermov::cartoons)
                .service(servermov::chucknorris)
                .service(servermov::comedy)
                .service(servermov::drama)
                .service(servermov::documentary)
                .service(servermov::fantasy)
                .service(servermov::godzilla)
                .service(servermov::harrypotter)
                .service(servermov::indianajones)
                .service(servermov::jamesbond)
                .service(servermov::johnwayne)
                .service(servermov::johnwick)
                .service(servermov::jurassicpark)
                .service(servermov::kingsmen)
                .service(servermov::meninblack)
                .service(servermov::misc)
                .service(servermov::nicolascage)
                .service(servermov::pirates)
                .service(servermov::riddick)
                .service(servermov::starwars)
                .service(servermov::startrek)
                .service(servermov::superheroes)
                .service(servermov::scifi)
                .service(servermov::tomcruize)
                .service(servermov::transformers)
                .service(servermov::tremors)
                .service(servermov::therock)
                .service(servermov::xmen)

                .service(servertvs::fuubar)
                .service(servertvs::houseofthedragon)
                .service(servertvs::ringsofpower)
                .service(servertvs::wheeloftime)
                .service(servertvs::voyager)
                .service(servertvs::sttv)
                .service(servertvs::enterprise)
                .service(servertvs::tng)
                .service(servertvs::discovery)
                .service(servertvs::picard)
                .service(servertvs::lowerdecks)
                .service(servertvs::prodigy)
                .service(servertvs::strangenewworlds)
                .service(servertvs::andor)
                .service(servertvs::badbatch)
                .service(servertvs::bobafett)
                .service(servertvs::obiwankenobi)
                .service(servertvs::mandalorian)
                .service(servertvs::talesofthejedi)
                .service(servertvs::visions)
                .service(servertvs::silo)
                .service(servertvs::thelastofus)
                .service(servertvs::foundation)
                .service(servertvs::alienworlds)
                .service(servertvs::alteredcarbon)
                .service(servertvs::cowboybebop)
                .service(servertvs::forallmankind)
                .service(servertvs::lostinspace)
                .service(servertvs::raisedbywolves)
                .service(servertvs::nightsky)
                .service(servertvs::orville)
                .service(servertvs::halo)
                .service(servertvs::shehulk)
                .service(servertvs::hford1923)
                .service(servertvs::secretinvasion)
                .service(servertvs::falconandthewintersoldier)
                .service(servertvs::reacher)


                // .service(crate::func::insert_review)
                // .route(
                //     "/hey",
                //     web::get().to(crate::func::server_functions::manual_hello),
                // )
                .service(fs::Files::new("/thumbnails", thumb_path.clone()).show_files_listing())
        })
        .bind(("192.168.0.94", 8080))?
        .run()
        .await?;
    }
    Ok(())
}
