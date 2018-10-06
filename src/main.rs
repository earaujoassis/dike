#![feature(rustc_private)]

extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
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
mod http_adaptor;
mod middlewares;

use dotenv::dotenv;
use http_adaptor::HttpAdaptor;
use utils::logger_factory;
use utils::pool_factory;

fn main() {
    dotenv().ok();

    let logger = logger_factory();
    let pool = pool_factory(&logger);
    let http_server = HttpAdaptor::new(&logger, &pool);
    http_server.start_http("localhost", "3000");
}
