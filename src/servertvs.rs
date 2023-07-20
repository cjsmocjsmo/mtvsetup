use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;

#[get("/comedy/fuubar/{season}")]
pub async fn fuubar(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("FooBar");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/fantasy/houseofthedragon/{season}")]
pub async fn houseofthedragon(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("House Of The Dragon");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/fantasy/ringsofpower/{season}")]
pub async fn ringsofpower(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Rings Of Power");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/fantasy/wheeloftime/{season}")]
pub async fn wheeloftime(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Wheel Of Time");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/voyager/{season}")]
pub async fn voyager(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Voyager");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/sttv/{season}")]
pub async fn sttv(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("STTV");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/enterprise/{season}")]
pub async fn enterprise(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("ENT");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/tng/{season}")]
pub async fn tng(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("TNG");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/discovery/{season}")]
pub async fn discovery(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Discovery");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/picard/{season}")]
pub async fn picard(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Picard");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/lowerdecks/{season}")]
pub async fn lowerdecks(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Lower Decks");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/prodigy/{season}")]
pub async fn prodigy(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Prodigy");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/strangenewworlds/{season}")]
pub async fn strangenewworlds(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Strange New Worlds");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/andor/{season}")]
pub async fn andor(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Andor");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/badbatch/{season}")]
pub async fn badbatch(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Bad Batch");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/bobafett/{season}")]
pub async fn bobafett(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("BobbaFett");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/mandalorian/{season}")]
pub async fn mandalorian(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Mandalorian");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/talesofthejedi/{season}")]
pub async fn talesofthejedi(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Tales Of The Jedi");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/obiwan/{season}")]
pub async fn obiwankenobi(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Obi Wan");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/visions/{season}")]
pub async fn visions(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Visions");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/silo/{season}")]
pub async fn silo(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Silo");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/thelastofus/{season}")]
pub async fn thelastofus(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("The Last Of Us");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/foundation/{season}")]
pub async fn foundation(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Foundation");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/science/alienworlds/{season}")]
pub async fn alienworlds(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Alien Worlds");
    let season = path.into_inner();
    println!("catagory: {}", catagory);
    println!("season: {}", season);
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/alteredcarbon/{season}")]
pub async fn alteredcarbon(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Altered Carbon");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/cowboybebop/{season}")]
pub async fn cowboybebop(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Cowboy Bebop");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/forallmankind/{season}")]
pub async fn forallmankind(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("For All Mankind");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/lostinspace/{season}")]
pub async fn lostinspace(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Lost In Space");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/raisedbywolves/{season}")]
pub async fn raisedbywolves(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Raised By Wolves");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/nightsky/{season}")]
pub async fn nightsky(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Night Sky");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/orville/{season}")]
pub async fn orville(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Orville");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/halo/{season}")]
pub async fn halo(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Halo");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/science/prehistoricplanet/{season}")]
pub async fn prehistoricplanet(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Prehistoric Planet 2022");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/secretinvasion/{season}")]
pub async fn secretinvasion(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Secret Invasion");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/loki/{season}")]
pub async fn loki(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Loki");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/iamgroot/{season}")]
pub async fn iamgroot(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("I Am Groot");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/falconandthewintersoldier/{season}")]
pub async fn falconandthewintersoldier(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Falcon And The Winter Soldier");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/msmarvel/{season}")]
pub async fn msmarvel(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Ms Marvel");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/wandavision/{season}")]
pub async fn wandavision(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Wandavision");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/moonknight/{season}")]
pub async fn moonknight(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Moonknight");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/hawkeye/{season}")]
pub async fn hawkeye(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Hawkeye");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/superheroes/shehulk/{season}")]
pub async fn shehulk(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Shehulk");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/western/hford1923/{season}")]
pub async fn hford1923(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Hford1923");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/crap/reacher/{season}")]
pub async fn reacher(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Reacher");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

async fn get_shows(cat: String, sea: String) -> String {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM tvshows ORDER BY episode ASC WHERE catagory = ?1 AND season = ?2")
        .unwrap();
    let mut rows = stmt.query(&[&cat, &sea]).expect("Unable to query db");
    let mut result = Vec::new();
    while let Some(row) = rows.next().expect("Unable to get next row") {
        let show = crate::setup::mtv_types::TVShow {
            id: row.get(0).expect("Unable to get id"),
            tvid: row.get(1).expect("Unable to get tvid"),
            size: row.get(2).expect("Unable to get size"), 
            catagory: row.get(3).expect("Unable to get catagory"),
            name: row.get(4).expect("Unable to get name"),
            season: row.get(5).expect("Unable to get season"),
            episode: row.get(6).expect("Unable to get episode"),
            path: row.get(7).expect("Unable to get path"),
            idx: row.get(8).expect("Unable to get idx"),
        };
        result.push(show);
    }
    let result = serde_json::to_string(&result).unwrap();

    result
}
