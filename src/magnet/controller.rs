extern crate rocket;
// extern crate rocket_contrib;
extern crate serde_json;
extern crate tera;

use self::tera::Context;
use rocket_contrib::{Template, JSON};

use db;
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
fn search(query: String, page: i32, size: i32, conn: db::Conn) -> JSON<Message> {
    JSON(Message{
        search_string: query.clone(),
        magnets_total: Magnet::count(&String::from(""), &conn),
        magnets_filtered: Magnet::count(&query, &conn),
        magnets: Magnet::search(&query, page, size, &conn)
    })
}
