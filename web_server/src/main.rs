#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::*;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::{Serialize, Deserialize};
use std::io;
use rocket::response::NamedFile;
use std::path::PathBuf;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Command {
    name: String
}

fn main() {

    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, static_file, new_command])
        .mount("/api", routes![hello])
        .attach(Template::fairing())
        .launch();
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
#[get("/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("frontend/static/").join(path)).ok()
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[post("/command", data = "<command_form>")]
fn new_command(command_form: rocket_contrib::json::Json<Command>) -> String {
    format!("Command received: {:?}", command_form)
}
#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("frontend/static/index.html")
}
