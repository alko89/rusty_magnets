#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
// #[macro_use] extern crate diesel;
// #[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tera;
extern crate rusty_magnets;

use rusty_magnets::static_files;
use rusty_magnets::db;
use rusty_magnets::magnet;

use self::tera::Context;
use rocket_contrib::{Template, Json};
use magnet::Magnet;


#[derive(Serialize, Deserialize)]
struct Message {
    search_string: String,
    magnets_total: i64,
    magnets_filtered: i64,
    magnets: Vec<Magnet>
}

// #[derive(Debug, Serialize)]
// struct Context {
//     magnets: Vec<Magnet>
// }
// 
// impl Context {
//     pub fn all(conn: &db::Conn) -> Context {
//         Context{
//             magnets: Magnet::all(conn)
//         }
//     }
// }


#[get("/")]
fn list() -> Template {
    Template::render("list", &Context::new())
}

#[get("/search/<query>/<page>/<size>")]
fn search(query: String, page: i32, size: i32, conn: db::Conn) -> Json<Message> {
    Json(Message{
        search_string: query.clone(),
        magnets_total: Magnet::count(&String::from(""), &conn),
        magnets_filtered: Magnet::count(&query, &conn),
        magnets: Magnet::search(&query, page, size, &conn)
    })
}

fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![list, static_files::all])
        .mount("/magnet", routes![list, search])
        .attach(Template::fairing())
        .launch();
}
