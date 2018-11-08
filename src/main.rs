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
pub mod utils;
pub mod datastore;
pub mod controllers;
pub mod middlewares;

use dotenv::dotenv;
use actix_web::server;

use utils::logger_factory;
use utils::pool_factory;
use utils::define_endpoints;

fn main() {
    dotenv().ok();

    let logger = logger_factory();
    let pool = pool_factory(&logger);
    let address = format!("{}:{}", "localhost", "3000");

    {
        info!(logger, "Server Running"; o!("address" => address.clone()));
    }

    let server = server::new(move || {
        define_endpoints(&logger, &pool)
    });

    server.bind(address).unwrap().run();
}
