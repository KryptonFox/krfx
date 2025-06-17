use actix_web::middleware::from_fn;
use crate::handlers::api::auth::login::login_handler;
use crate::handlers::api::auth::signup::signup_handler;
use crate::handlers::api::record::create_link::create_link_handler;
use crate::handlers::api::record::upload_file::upload_file_handler;
use actix_web::web;
use crate::middlewares::auth_middleware;

pub(super) fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login_handler)
            .service(signup_handler),
    );
    cfg.service(
        web::scope("/record")
            .wrap(from_fn(auth_middleware))
            .service(create_link_handler)
            .service(upload_file_handler),
    );
}
