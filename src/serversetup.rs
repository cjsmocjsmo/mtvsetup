use actix_web::{get, web, HttpResponse, Responder};

#[get("/setup")]
pub async fn run_setup(path: web::Path<String>) -> impl Responder {
    let _vars = crate::envvars::set_env_vars();
    let _setup = crate::setup::setup();
    let result = format!("Setup is complete.\n Go to http://{} is complete!", path);
    HttpResponse::Ok().body(result)
}