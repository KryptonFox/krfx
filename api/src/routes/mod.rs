use crate::routes::api::configure_api_routes;
use crate::routes::link::configure_link_route;
use actix_web::web;

mod api;
mod link;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(configure_link_route);
    cfg.service(web::scope("/api").configure(configure_api_routes));
}
