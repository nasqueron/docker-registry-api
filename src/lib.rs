#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;

extern crate rocket_contrib;

pub mod app;
pub mod registry;
pub mod requests;
