#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello and welcome to my custom search engine!"
}

#[get("/search?<cmd>")]
fn search(cmd: &RawStr) -> Redirect {
    println!("The cmd is: {}", cmd);
    match cmd.as_str() {
        "mail" => Redirect::to("https://mail.google.com/"),
        "tw" => Redirect::to("https://twitter.com/"),
        "you" => Redirect::to("https://youtube.com/"),
        _ => Redirect::to("https://google.com/")
    }
}

// Keeping in for testing purposes
#[get("/hello?<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

fn main() {
   rocket::ignite().mount("/", routes![index, search, hello]).launch();
}
