#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::process::Command;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket::http::RawStr;

#[get("/wifis")]
fn index() -> Json<Vec<String>> {

    let mut echo_hello = Command::new("ls");
    echo_hello.arg("-la");
    let hello_1 = echo_hello.output().expect("failed to execute process");

    let output = String::from_utf8_lossy(&hello_1.stdout);

    let mut lines : Vec<String> = Vec::new();

    for line in output.as_ref().lines() {
        lines.push(String::from(line));
    }

    return Json(lines);
}

#[post("/wifi/<ssid>/connect")]
fn connect(ssid : &RawStr) -> String{
    return String::from("test");
}


fn main(){
    rocket::ignite().mount("/api", routes![index]).mount("/", StaticFiles::from("static")).launch();
}