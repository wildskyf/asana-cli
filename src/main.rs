extern crate curl;
extern crate rustc_serialize;

use std::env;
use std::io::{stdout}; // Write
use std::fs::File;
use std::io::prelude::*;
use curl::easy::{Easy, List};
use rustc_serialize::json::Json;

fn show(token: &str, target: &str) {
    // println!("Here are your projects:");

    let mut data = Vec::new();

    let mut easy = Easy::new();
    let header_string = format!("Authorization: Bearer {}", token);
    let mut list = List::new();

    easy.url("https://app.asana.com/api/1.0/users/me").unwrap();
    list.append(&header_string).unwrap();
    easy.http_headers(list).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let res = String::from_utf8(data).unwrap();

    let json_obj = Json::from_str(&res).unwrap();

    /* {
        "data": {
            "email"
            "id"
            "name"
            "photo": {
                "image_128x128"
                "image_21x21"
                "image_27x27"
                "image_36x36"
                "image_60x60"
            },
            "workspaces": [{
                "id"
                "name"
            }]
        }
    } */


    println!("{}", json_obj["data"]["workspaces"]);

}

fn main() {

    let mut file = File::open(".token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).unwrap();


    let version = "1.0.0";
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("_(:3 ﾞ)_");
    }
    else {
        if args[1] == "version" {
            println!("{}", version);
        }
        else if args[1] == "show" {

            if args.len() == 2 {
                println!("Show what?");
            }
            else if args[2] == "tasks" {
                println!("There are too many tasks. You won't want to see them. ;)");
            }
            else {
                show(&token, &args[2]);
            }
        }
        else {
            println!("{}",args[1]);
        }
    }
}
