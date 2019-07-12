#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::process::Command;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket::http::RawStr;

use serde::Serialize;


#[get("/wifis")]
fn index() -> Json<Vec<Wifi>> {

    let mut echo_hello = Command::new("ls");
    echo_hello.arg("-la");
    let hello_1 = echo_hello.output().expect("failed to execute process");

    let output = String::from_utf8_lossy(&hello_1.stdout);

    let mut lines : Vec<Wifi> = Vec::new();

    for line in output.as_ref().lines() {
        //lines.push(String::from(line));

        lines.push(Wifi{ssid: String::from("SSID"), speed : String::from("1234"), security: String::from("WPA")});

    }

    return Json(lines);
}

#[derive(Serialize)]
pub struct Wifi {
    pub ssid : String,
    pub speed : String,
    pub security : String
}

#[post("/wifi/<ssid>/connect")]
fn connect(ssid : &RawStr) -> String{
    return String::from("test");
}


fn main(){
    rocket::ignite().mount("/api", routes![index]).mount("/", StaticFiles::from("static")).launch();
}