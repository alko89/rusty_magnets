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

mod static_files;
mod db;
mod magnet;

use magnet::{controller as magnet_controller};
use rocket_contrib::Template;


fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![magnet_controller::list, static_files::all])
        .mount("/magnet", routes![magnet_controller::list, magnet_controller::search])
        .attach(Template::fairing())
        .launch();
}
