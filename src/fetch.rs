use {std, serde_json};
use curl::easy::{Easy, List};
use serde_json::Value;


pub fn fetch_api(url: &str, token: &str) -> Value {
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
