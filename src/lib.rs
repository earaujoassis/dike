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

pub mod controllers;
pub mod datastore;
pub mod middlewares;
pub mod utils;
