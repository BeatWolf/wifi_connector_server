#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::process::Command;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket::http::RawStr;

use serde::Serialize;
use std::collections::HashMap;
use std::ops::Deref;

#[get("/wifis")]
fn index() -> Json<Vec<CustomWifi>> {

    let mut echo_hello = Command::new("nmcli");
    echo_hello.arg("-c").arg("no").arg("--terse").arg("dev").arg("wifi");
    let hello_1 = echo_hello.output().expect("failed to execute process");

    let output = String::from_utf8_lossy(&hello_1.stdout);

    let mut wifis = HashMap::new();

    for line in output.as_ref().lines() {
        //lines.push(String::from(line));

        if !line.starts_with("IN-USE") {
            let split_line = line.split(":").collect::<Vec<&str>>();

            /*for i in 0..split_line.len(){
                print!("{} {} ", i, split_line[i]);
            }
            println!();*/

            if split_line.len() > 6 && split_line[1].len() > 0{
                wifis.insert(String::from(split_line[1]), CustomWifi{ssid: String::from(split_line[1]), speed : String::from(split_line[4]), security: String::from(split_line[7])});
            }
        }

    }

    let mut wifi_vec = Vec::new();

    for(_, val) in wifis.iter(){
        wifi_vec.push(val.clone());
    }

    return Json(wifi_vec);
}

#[derive(Serialize, Clone)]
pub struct CustomWifi {
    pub ssid : String,
    pub speed : String,
    pub security : String
}

#[get("/wifi/<ssid>/connect")]
fn connect(ssid : &RawStr) -> String{
    return String::from("test");
}


fn main(){
    rocket::ignite().mount("/api", routes![index, connect]).mount("/", StaticFiles::from("static")).launch();
}