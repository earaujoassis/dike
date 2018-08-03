#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[allow(unused)]
pub mod data;
#[allow(unused)]
pub mod web;
#[allow(unused)]
pub mod power_dns;
