use crate::handlers::link::link_handler;
use actix_web::web;

pub(super) fn configure_link_route(cfg: &mut web::ServiceConfig) {
    cfg.service(link_handler);
}
