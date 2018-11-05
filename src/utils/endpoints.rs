use std::vec::Vec;
use actix_web::App;
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
            .resource("/adddomainkey/:domain/:domain_id", |r| r.f(PowerDNSController::dns_add_domain_key))
            .resource("/getdomainkeys/:domain/:domain_id", |r| r.f(PowerDNSController::dns_get_domain_keys))
            .resource("/lookup/:domain/:qtype", |r| r.f(PowerDNSController::dns_lookup))
            .resource("/list/:domain_id/:domain", |r| r.f(PowerDNSController::dns_list)),
        App::new()
            .middleware(LoggerMiddleware::new(&logger))
            .middleware(DieselMiddleware::new(&logger, &pool))
            .resource("/", |r| r.f(ServicesController::index))
            .resource("/ping", |r| r.f(ServicesController::ping))
            .resource("/clients", |r| r.f(ServicesController::clients))
    ]
}
