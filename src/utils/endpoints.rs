use std::vec::Vec;
use actix_web::{http::Method, App};
use slog::Logger;

use controllers::power_dns as PowerDNSController;
use controllers::services as ServicesController;

use utils::pool::DieselPool;
#[allow(unused_imports)]
use middlewares::GetSaltMiddleware;
use middlewares::DieselMiddleware;
use middlewares::LoggerMiddleware;

pub fn define_endpoints(logger: &Logger, pool: &DieselPool) -> Vec<App> {
    vec![
        App::new()
            .prefix("/dns")
            .middleware(LoggerMiddleware::new(&logger))
            .middleware(DieselMiddleware::new(&logger, &pool))
            .resource("/adddomainkey/:domain/:domain_id", |r| r.method(Method::GET).f(PowerDNSController::dns_add_domain_key))
            .resource("/getdomainkeys/:domain/:domain_id", |r| r.method(Method::GET).f(PowerDNSController::dns_get_domain_keys))
            .resource("/lookup/:domain/:qtype", |r| r.method(Method::GET).f(PowerDNSController::dns_lookup))
            .resource("/list/:domain_id/:domain", |r| r.method(Method::GET).f(PowerDNSController::dns_list)),
        App::new()
            .prefix("/clients")
            .middleware(LoggerMiddleware::new(&logger))
            .middleware(DieselMiddleware::new(&logger, &pool))
            .resource("/update", |r| r.method(Method::POST).f(ServicesController::clients_update))
            .resource("/active", |r| r.method(Method::GET).f(ServicesController::clients_active)),
        App::new()
            .middleware(LoggerMiddleware::new(&logger))
            .middleware(DieselMiddleware::new(&logger, &pool))
            .resource("/", |r| r.method(Method::GET).f(ServicesController::index))
            .resource("/ping", |r| r.method(Method::GET).f(ServicesController::ping))
    ]
}
