use actix_web::{get, web, Result, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;




#[get("/comedy/fuubar/{season}")]
pub async fn fuubar(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("fuubar");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}



#[get("/fantasy/houseofthedragon/{season}")]
pub async fn houseofthedragon(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("houseofthedragon");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/fantasy/ringsofpower/{season}")]
pub async fn ringsofpower(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("ringsofpower");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/fantasy/wheeloftime/{season}")]
pub async fn wheeloftime(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("wheeloftime");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}



#[get("/startrek/voyager/{season}")]
pub async fn voyager(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("voyager");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/sttv/{season}")]
pub async fn sttv(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("sttv");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/enterprise/{season}")]
pub async fn enterprise(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("enterprise");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/tng/{season}")]
pub async fn tng(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("tng");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/discovery/{season}")]
pub async fn discovery(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("discovery");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/picard/{season}")]
pub async fn picard(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("picard");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/lowerdecks/{season}")]
pub async fn lowerdecks(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("lowerdecks");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/prodigy/{season}")]
pub async fn prodigy(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("prodigy");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/startrek/strangenewworlds/{season}")]
pub async fn strangenewworlds(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("strangenewworlds");
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/andor/{season}")]
pub async fn andor(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("andor");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/badbatch/{season}")]
pub async fn badbatch(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("badbatch");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/bobafett/{season}")]
pub async fn bobafett(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("bobafett");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/mandalorian/{season}")]
pub async fn mandalorian(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("mandalorian");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/talesofthejedi/{season}")]
pub async fn talesofthejedi(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("talesofthejedi");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/obiwankenobi/{season}")]
pub async fn obiwankenobi(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("obiwankenobi");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/starwars/visions/{season}")]
pub async fn visions(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("visions");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/silo/{season}")]
pub async fn silo(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("silo");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/thelastofus/{season}")]
pub async fn thelastofus(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("thelastofus");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/foundation/{season}")]
pub async fn foundation(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("foundation");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/alienworlds/{season}")]
pub async fn alienworlds(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("alienworlds");
    let season = path.into_inner();
    println!("catagory: {}", catagory);
    println!("season: {}", season);
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/alteredcarbon/{season}")]
pub async fn alteredcarbon(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("alteredcarbon");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/cowboybebop/{season}")]
pub async fn cowboybebop(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("cowboybebop");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/forallmankind/{season}")]
pub async fn forallmankind(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("forallmankind");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/lostinspace/{season}")]
pub async fn lostinspace(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("lostinspace");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/raisedbywolves/{season}")]
pub async fn raisedbywolves(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("raisedbywolves");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/nightsky/{season}")]
pub async fn nightsky(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("nightsky");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/orville/{season}")]
pub async fn orville(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("orville");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/scifi/halo/{season}")]
pub async fn halo(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("halo");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/science/prehistoricplanet/{season}")]
pub async fn prehistoricplanet(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("prehistoricplanet");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/superheroes/secretinvasion/{season}")]
pub async fn secretinvasion(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("prehistoricplanet");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/superheroes/loki/{season}")]
pub async fn loki(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("loki");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/superheroes/iamgroot/{season}")]
pub async fn iamgroot(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("iamgroot");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}


#[get("/superheroes/falconandthewintersoldier/{season}")]
pub async fn falconandthewintersoldier(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("falconandthewintersoldier");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}


#[get("/superheroes/msmarvel/{season}")]
pub async fn msmarvel(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("msmarvel");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/superheroes/wandavision/{season}")]
pub async fn wandavision(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("wandavision");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[get("/superheroes/moonknight/{season}")]
pub async fn moonknight(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("moonknight");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}


#[get("/superheroes/hawkeye/{season}")]
pub async fn hawkeye(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("hawkeye");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}


#[get("/superheroes/shehulk/{season}")]
pub async fn shehulk(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("shehulk");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}


#[get("/western/hford1923/{season}")]
pub async fn hford1923(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("hford1923");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await.unwrap();
    HttpResponse::Ok().json(result)
}

async fn get_shows(cat: String, sea: String) -> Result<String> {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM tvshows WHERE catagory = ?1 AND season = ?2")
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
    Ok(result)
}