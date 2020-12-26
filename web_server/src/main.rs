#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::*;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(FromForm)]
struct Command {
    name: String
}

fn main() {

    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index])
        .mount("/api", routes![hello])
        .attach(Template::fairing())
        .launch();
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[post("/command", data = "<command_form>")]
fn new_command(command_form: Form<Command>) -> String {
    let command: Command = command_form.into_inner();
    format!("Command received: {:?}", command.name)
}

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String
    }

    let context = Context {
        first_name: String::from("Kristof"),
        last_name: String::from("")
    };
    Template::render("home", context)
}