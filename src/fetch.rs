use {std, serde_json};
use curl::easy::{Easy, List};
use std::io::Read;
use serde_json::Value;

pub fn get_api(url: &str, token: &str) -> Value {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    let header_str = format!("Authorization: Bearer {}", token);
    let mut list = List::new();

    // FIXME: unwrap => unwrap_or_else
    easy.url(&url).unwrap();
    list.append("Asana-Fast-Api: true").unwrap();
    list.append(&header_str).unwrap();
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

pub fn post_api(url: &str, token: &str, post_data_str: &str) -> Value {
    // FIXME: unwrap => unwrap_or_else
    let header_str = format!("Authorization: Bearer {}", token);
    let mut post_data = post_data_str.as_bytes();
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(post_data.len() as u64).unwrap();

    let mut list = List::new();
    list.append("Asana-Fast-Api: true").unwrap();
    list.append(&header_str).unwrap();
    easy.http_headers(list).unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(post_data.read(buf).unwrap_or(0))
        }).unwrap();
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
