#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;

#[allow(unused)]
pub mod data;
#[allow(unused)]
pub mod web;
#[allow(unused)]
pub mod power_dns;
#[allow(unused)]
pub mod schema;
