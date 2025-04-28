mod auth;
mod link;
mod record;
mod types;
mod utlis;

use crate::auth::login::login;
use crate::auth::signup::signup;
use crate::link::link_service;
use crate::record::create_link::create_link;
use crate::record::upload::upload;
use crate::types::{AppState, Environment};
use actix_cors::Cors;
use actix_multipart::form::MultipartFormConfig;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::redirect("/", "https://web.krfx.ru").permanent());
    cfg.service(web::resource("/favicon.ico").route(web::to(|| HttpResponse::NotFound())));
    cfg.service(link_service);
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(signup)
            .service(login)
            .service(web::scope("/record").service(upload).service(create_link)),
    );
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let env = envy::from_env::<Environment>().ok().unwrap();
    let state = AppState::new(&env).await;

    HttpServer::new(move || {
        let cors = Cors::permissive(); // TODO normal CORS for production

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(config)
            .configure(api_config)
            .app_data(web::Data::new(state.clone()))
            .app_data(MultipartFormConfig::default().total_limit(100 * 1024 * 1024))
            .app_data(web::Data::new(awc::Client::default()))
    })
    .bind((env.host.as_str(), env.port))?
    .run()
    .await
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
