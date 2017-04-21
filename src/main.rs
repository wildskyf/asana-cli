extern crate curl;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use curl::easy::{Easy, List};
use rustc_serialize::json::Json;

fn fetch_api (url: &str, token: &str) -> String {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    let header_string = format!("Authorization: Bearer {}", token);
    let mut list = List::new();

    easy.url(&url).unwrap();
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
    String::from_utf8(data).unwrap()
}

fn show(token: &str, target: &str, options: &str) {
    match target {
        "workspaces" => {
            let res = fetch_api("https://app.asana.com/api/1.0/workspaces", &token);
            let json_obj = Json::from_str(&res).unwrap();

            let workspaces = json_obj["data"].as_array();

            match workspaces {
                Some(ref w) => {
                    for x in w.iter() {
                        println!("{} {}", x["id"], x["name"]);
                    }
                },
                None => println!("NOOOOOOOOO, {:?}", workspaces),
            }

        },

        "projects" => {

            // TODO: allow user to set default workspace
            let res = fetch_api("https://app.asana.com/api/1.0/projects", &token);
            let json_obj = Json::from_str(&res).unwrap();

            let workspaces = json_obj["data"].as_array();

            match workspaces {
                Some(ref w) => {
                    for x in w.iter() {
                        if (options != "") && x["name"].to_string().contains(&options) {
                            println!("{} {}", x["id"], x["name"]);
                        }
                        else if options == "" {
                            println!("{} {}", x["id"], x["name"]);
                        }
                    }
                },
                None => println!("NOOOOOOOOO, {:?}", workspaces),
            }


        },

        _ => {
            println!("Not supported target: {}", &target);
        }
    }
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
                if args.len() == 3 {
                    show(&token, &args[2], "");
                }
                else {
                    show(&token, &args[2], &args[3]);
                }
            }
        }
        else {
            println!("{}",args[1]);
        }
    }
}
