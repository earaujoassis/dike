#![feature(rustc_private)]
#![allow(proc_macro_derive_resolution_fallback)]
#![allow(dead_code)]

extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;
extern crate rustc_serialize;
extern crate base64;

#[macro_use]
mod utils;
mod datastore;
mod controllers;
mod middlewares;

use dotenv::dotenv;
use actix_web::{server, App};

use controllers::power_dns as PowerDNSController;
use controllers::services as ServicesController;

use utils::logger_factory;
use utils::pool_factory;
#[allow(unused_imports)]
use middlewares::GetSaltMiddleware;
use middlewares::DieselMiddleware;
use middlewares::LoggerMiddleware;

fn main() {
    dotenv().ok();

    let logger = logger_factory();
    let pool = pool_factory(&logger);
    let address = format!("{}:{}", "localhost", "3000");

    {
        info!(logger, "Server Running"; o!("address" => address.clone()));
    }

    let server = server::new(move || {
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
    });

    server.bind("localhost:3000").unwrap().run();
}
