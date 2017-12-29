extern crate curl;
extern crate serde_json;
extern crate clap;

mod fetch;

use std::process;
use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
use clap::{Arg, App, SubCommand};

// PROGRAM INFO
static TOKEN_FILE_NAME: &'static str = ".token";
static DEFAULT_WROKSPACE_FILE_NAME: &'static str = ".default_workspace";
static VERSION: &'static str = "1.0.0";

struct Config {
    token: String,
    default_ws: String
}

fn open_and_read(file_name: &str) -> String {
    let mut t = String::new();
    let mut file = File::open(file_name).unwrap_or_else(|_| {
        panic!("Asana init: {}: No such file or directory.", file_name);
    });
    file.read_to_string(&mut t).unwrap_or_else(|_| { panic!("Error happened when reading file.") } );
    t.trim().to_string()
}

fn print_workspace_name(config: &Config) {
    let url = format!("https://app.asana.com/api/1.0/workspaces/{}", &config.default_ws);

    println!("On workspace {}", fetch::fetch_api(&url, &config.token).as_object()
        .and_then(|obj| obj.get("data"))
        .and_then(|obj| obj.as_object())
        .and_then(|obj| obj.get("name"))
        .and_then(|obj| obj.as_string())
        .unwrap_or_else(|| {
            panic!("Failed to get 'data' value from json");
        })
    );
}

fn parse_task(d: &Value) -> (&str, &str, bool) {
    let task = d.as_object();
    let assignee_status = task.and_then(|task| task.get("assignee_status") ).unwrap().as_string().unwrap();
    let name            = task.and_then(|task| task.get("name") ).unwrap().as_string().unwrap();
    let completed       = task.and_then(|task| task.get("completed") ).unwrap().as_boolean().unwrap();

    (assignee_status, name, completed)
}

fn show_task_by_category(category: &str, data: &Vec<Value>, show_all: bool) {
    println!("{}:", category);
    for d in data.iter() {
        let (assignee_status, name, completed) = parse_task(&d);

        if assignee_status == category && name != "" {
            if show_all {
                let sec_indent = if name.chars().last().unwrap() == ':' { "  " } else { "     " } ;
                let check = if name.chars().last().unwrap() == ':' {
                    ""
                }
                else {
                    if completed { "[v]" } else { "[ ]" }
                };

                println!("{}{} {}", sec_indent, check, name);
            }
            else {
                 if !completed && name.chars().last().unwrap() != ':' {
                     println!("\t{}", name);
                 }
            }
        }
    }
}

fn show_my_tasks(config: &Config, show_all: bool) {
    // TODO: allow user to set default workspace
    let url = format!("https://app.asana.com/api/1.0/tasks?workspace={}&assignee=me&opt_fields=assignee_status,name,completed", &config.default_ws);
    let json_obj:Value = fetch::fetch_api(&url, &config.token);

    let data = json_obj.as_object()
        .and_then(|obj| obj.get("data"))
        .and_then(|data| data.as_array())
        .unwrap_or_else(|| {
            panic!("Failed to access 'data': undefined");
        });

    show_task_by_category("today", data, show_all);
    show_task_by_category("upcoming", data, show_all);
}


fn asana_add(_config: Config, task_name: &str) {
    println!("Let's add task assigned to you: {}", task_name);
}

fn asana_status(config: Config, show_all: bool) {
    print_workspace_name(&config);
    println!("Here are tasks assigned to you:");
    show_my_tasks(&config, show_all);
}

fn main() {
    let matches = App::new("asana")
        .version(VERSION)
        .author("Wildsky F. <wildsky@moztw.org>")
        .about("Yet Another Asana Client")
        .subcommand(SubCommand::with_name("add")
                    .about("add task (default assign to yourself)")
                    .arg(Arg::with_name("TASK_NAME")
                         .required(true)
                         .help("Sets the task to add")))
        .subcommand(SubCommand::with_name("status")
                    .about("show your uncompleted tasks")
                    .arg(Arg::with_name("all")
                         .short("a")
                         .long("all")
                         .help("show all task assigned to you, completed and uncompleted (with prefix [ ] or [v])")))
        .subcommand(SubCommand::with_name("tasks").about(""))
        .get_matches();

    let config = Config {
        token: open_and_read(TOKEN_FILE_NAME),
        default_ws: open_and_read(DEFAULT_WROKSPACE_FILE_NAME)
    };

    if let Some(matches) = matches.subcommand_matches("status") {
        asana_status(config, matches.is_present("all"));
    }
    else if let Some(matches) = matches.subcommand_matches("add") {
        asana_add(config, matches.value_of("TASK_NAME").unwrap());
    }
    else if let Some(_) = matches.subcommand_matches("tasks") {
        println!("There are too many tasks. You won't want to see them all. ;)")
    }

    process::exit(0)
}
