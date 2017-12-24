extern crate curl;
extern crate serde_json;
#[macro_use]
extern crate clap;

use std::process;
use std::fs::File;
use std::io::prelude::*;
use curl::easy::{Easy, List};
use serde_json::Value;

// PROGRAM INFO
static TOKEN_FILE_NAME: &'static str = ".token";
static DEFAULT_WROKSPACE_FILE_NAME: &'static str = ".default_workspace";
static VERSION: &'static str = "1.0.0";

fn open_and_read(file_name: &str, taker: &mut String) {
    let mut file = File::open(file_name).unwrap_or_else(|_| {
        panic!("Asana init: {}: No such file or directory.", file_name);
    });
    file.read_to_string(taker).unwrap_or_else(|_| { panic!("Error happened when reading file.") } );
}

fn fetch_api(url: &str, token: &str) -> Value {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    let header_string = format!("Authorization: Bearer {}", token);
    let mut list = List::new();

    // FIXME: unwrap => unwrap_or_else
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

    let body = std::str::from_utf8(&data).unwrap_or_else(|e| {
        panic!("Failed to parse response from {}; error is {}", url, e);
    });

    serde_json::from_str(body).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    })
}

fn print_workspace_name(token: &str, workspace_id: &str) {
    let url = format!("https://app.asana.com/api/1.0/workspaces/{}", &workspace_id);

    println!("On workspace {}", fetch_api(&url, &token).as_object()
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

fn show_my_tasks(token: &str, workspace_id: &str) {

    // TODO: allow user to set default workspace
    let url = format!("https://app.asana.com/api/1.0/tasks?workspace={}&assignee=me&opt_fields=assignee_status,name,completed", &workspace_id);
    let json_obj:Value = fetch_api(&url, &token);

    let data = json_obj.as_object()
        .and_then(|obj| obj.get("data"))
        .and_then(|data| data.as_array())
        .unwrap_or_else(|| {
            panic!("Failed to get 'data' value from json");
        });

    println!("Today:");
    for d in data.iter() {
        let (assignee_status, name, completed) = parse_task(&d);

        if assignee_status == "today" && name != "" && !completed {
            println!("\t{}", name);
        }
    }

    println!("Upcoming:");
    for d in data.iter() {
        let (assignee_status, name, completed) = parse_task(&d);

        if assignee_status == "upcoming" && name != "" && !completed {
            println!("\t{}", name);
        }
    }
}

fn asana_status(token: &str, current_workspace_id: &str) {
    println!("Here are tasks assigned to you:");
    print_workspace_name(token, current_workspace_id);
    show_my_tasks(token, current_workspace_id);
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: VERSION)
        (author: "Wildsky F. <wildsky@moztw.org>")
        (about: "Yet Another Asana Client")
        (@subcommand status =>
            (about: "show your uncompleted tasks")
            (version: VERSION)
            (author: "Wildsky F. <wildsky@moztw.org>")
        )
        (@subcommand tasks =>
            (about: "")
        )
    ).get_matches();

    let mut token = String::new();
    let mut default_workspace_id = String::new();
    open_and_read(TOKEN_FILE_NAME, &mut token);
    open_and_read(DEFAULT_WROKSPACE_FILE_NAME, &mut default_workspace_id);
    let token = token.trim();
    let default_workspace_id = default_workspace_id.trim();

    if let Some(_) = matches.subcommand_matches("status") {
        asana_status(&token, &default_workspace_id)
    }
    else if let Some(_) = matches.subcommand_matches("tasks") {
        println!("There are too many tasks. You won't want to see them all. ;)")
    }

    process::exit(0)
}
