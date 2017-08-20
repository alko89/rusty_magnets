#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod static_files;
pub mod db;
pub mod magnet;
