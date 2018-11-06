extern crate dike;
extern crate dotenv;
extern crate actix_web;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::test::TestServer;

pub fn test_helper() -> TestServer {
    dotenv::from_filename(".env.testing").ok();
    embed_migrations!("migrations");

    let logger = dike::utils::logger_factory();
    let pool = dike::utils::pool_factory(&logger);
    let server = TestServer::with_factory(move || { dike::utils::define_endpoints(&logger, &pool) });

    server
}

pub mod controllers;
