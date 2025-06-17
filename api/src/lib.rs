mod config;
mod errors;
mod handlers;
mod middlewares;
mod routes;
mod services;
mod state;
mod types;
mod utils;

use crate::routes::configure_routes;
use crate::services::{AuthService, NameService, RecordService};
use crate::state::AppState;
use actix_cors::Cors;
use actix_multipart::form::MultipartFormConfig;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use std::sync::Arc;

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::redirect("/", "https://web.krfx.ru").permanent());
    cfg.service(web::resource("/favicon.ico").route(web::to(|| HttpResponse::ImATeapot())));
    cfg.configure(configure_routes);
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    // TODO new epoch for snowflakes
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // create AppState
    let state = Arc::new(AppState::new().await);

    // create record service
    let record_service = Arc::new(RecordService::new(state.clone()).await);

    // host and port to listening
    let host = state.env.host.clone();
    let port = state.env.port;

    HttpServer::new(move || {
        let cors = Cors::permissive(); // TODO normal CORS for production

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(config)
            .app_data(web::Data::from(record_service.clone()))
            .app_data(web::Data::new(AuthService::new(state.clone())))
            .app_data(web::Data::new(NameService::new(state.clone())))
            .app_data(MultipartFormConfig::default().total_limit(100 * 1024 * 1024))
            .app_data(web::Data::new(awc::Client::default()))
            .app_data(web::Data::from(state.clone()))
    })
    .bind((host, port))?
    .run()
    .await
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
