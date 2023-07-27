use actix_web::{get, web, HttpResponse, Responder};

#[get("/setup/{status}")]
pub async fn run_setup(path: web::Path<String>) -> impl Responder {
    let fuck = path.clone();
    let _status = path.into_inner();
    let _setup = crate::setup::setup();
    let result = format!("Go to http://{} is complete!", fuck.clone());
    HttpResponse::Ok().body(result)
}