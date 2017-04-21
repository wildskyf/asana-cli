extern crate curl;
extern crate rustc_serialize;

use std::process;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use curl::easy::{Easy, List};
use rustc_serialize::json::Json;

fn fetch_api (url: &str, token: &str) -> Json {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    let header_string = format!("Authorization: Bearer {}", token);
    let mut list = List::new();

    easy.url(&url).unwrap();
    list.append("Asana-Fast-Api: true").unwrap();
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
    Json::from_str(&res).unwrap()
}

fn show(token: &str, target: &str, options: &str) {
    // TODO: allow user to set default workspace
    let url = format!("https://app.asana.com/api/1.0/{}", &target);
    let json_obj = fetch_api(&url, &token);

    match json_obj["data"].as_array() {
        Some(ref w) => {
            for x in w.iter() {
                if (options != "") && x["name"].to_string().to_lowercase().contains(&options) {
                    println!("{} {}", x["id"], x["name"]);
                }
                else if options == "" {
                    println!("{} {}", x["id"], x["name"]);
                }
            }
        },
        None => println!("{}", &json_obj),
    }
}

fn print_help(args: &Vec<String>, is_error: bool) {

    if is_error {
        println!("");
        for i in 0..args.len() {
            print!("{} ", args[i]);
        }
        print!("is not work, FYI...\n");
    }

    println!("
        Asana Command Line Tool
        ==========================================\n
        options:
            --version, -v\tshow the version of this tool.\n
        Commands:
            workspaces   \tshow all workspaces you belong to.
            projects     \tshow all projects.
                --query, -q \tshow project contain the query string.
            users        \tshow all users.
            tasks        \t[not support yet] show all tasks.
    ");
}

fn main() {

    let mut file = File::open(".token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).unwrap();

    let version = "1.0.0";
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => print_help(&args, false),
        Some(arg1) => {
            match arg1.as_ref() {
                "--version" | "-v" => println!("{}", version),
                "tasks" => println!("There are too many tasks. You won't want to see them. ;)"),
                "workspaces" | "projects" | "users" => {
                    match args.get(2) {
                        None => show(&token, &args[1], ""), // show all projects
                        Some(arg2) => {
                            match arg2.as_ref() {
                                "-q" | "--query" => {
                                    match args.get(3) {
                                        None => print_help(&args, true),
                                        Some(arg3) => show(&token, &arg1, &(arg3.to_lowercase()))
                                    }
                                },
                                _ => print_help(&args, false)
                            }
                        }
                    }
                },
                "--help" | "-h" | _ => print_help(&args, false)
            }
        }
    }

    process::exit(0)
}
