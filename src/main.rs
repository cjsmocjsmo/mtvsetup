use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;


pub mod envvars;
pub mod servermov;
pub mod serversetup;
pub mod servertvs;
pub mod setup;

use env_logger::{Builder, Target};

#[actix_web::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .target(Target::Stdout)
        .init();

    log::info!("MTV Start");
    let _vars = envvars::set_env_vars();
    log::info!("Env Vars have been set");

    // if setup::mtv_tables::db_file_exists() == false {
    //     setup::mtv_tables::create_db_file();
    //     log::info!("created db file")
    // }
    let t_dir_exists = setup::mtv_image::thumbnail_dir_exists();

    if !t_dir_exists {
        setup::mtv_image::create_thumbnail_dir();
        log::info!("created thumb dir")
    }

    let thumb_path =
        env::var("MTV_MOVIES_THUMBNAIL_PATH").expect("MTV_MOVIES_THUMBNAIL_PATH not set");

    let socket = setup::mtv_utils::gen_server_addr();
    println!("To setup db go to: http://{}/setup/now", socket.clone());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(servermov::hello)
            .service(serversetup::run_setup)
            .service(serversetup::run_setup_check)
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
            .service(servermov::buzz)
            .service(servermov::charliebrown)
            .service(servermov::minions)
            .service(servermov::oldies)
            .service(servermov::tinkerbell)
            .service(servermov::stalone)
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
            .service(servertvs::alteredcarbon)
            .service(servertvs::cowboybebop)
            .service(servertvs::forallmankind)
            .service(servertvs::lostinspace)
            .service(servertvs::raisedbywolves)
            .service(servertvs::nightsky)
            .service(servertvs::orville)
            .service(servertvs::halo)
            .service(servertvs::secretinvasion)
            .service(servertvs::falconwintersoldier)
            .service(servertvs::hawkeye)
            .service(servertvs::iamgroot)
            .service(servertvs::loki)
            .service(servertvs::moonknight)
            .service(servertvs::msmarvel)
            .service(servertvs::shehulk)
            .service(servertvs::wandavision)
            .service(servertvs::hford1923)
            .service(servertvs::prehistoricplanet)
            .service(servertvs::ahsoka)
            .service(fs::Files::new("/thumbnails", thumb_path.clone()).show_files_listing())
    })
    .bind(socket)?
    .run()
    .await?;

    Ok(())
}
