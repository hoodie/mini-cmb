//! Tiny hyper+json example
//!
//! Cargo.toml:
//!
//! ```toml
//! [package]
//! name = "catch_my_bus2"
//! version = "0.1.0"
//! authors = ["Hendrik Sollich <hendrik@hoodie.de>"]
//!
//! [dependencies]
//! hyper = "*"
//! json = "*"
//! ```

extern crate hyper;
extern crate json;

use std::io::Read;

use hyper::client::Client;
use json::JsonValue;



fn station_url( station:&str) -> String{
    let city = "Dresden";
    let base_url = "http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do";
    format!("{base}?ort={city}&vz=0&hst={begin}",
                          base=base_url,
                          city=city,
                          begin=station)
}

/// Wrapper around `get_path()`.
///
/// Splits path string
/// and replaces `JsonValue::Null` and `JsonValue::BadValue`.
pub fn get<'a>(json:&'a JsonValue, key:&str) -> Option<&'a JsonValue>{
    match get_path(json, &key.split('/').filter(|k|!k.is_empty()).collect::<Vec<&str>>()) {
        Some(&JsonValue::Null)  => None,
        content => content
    }
}


/// Returns content at `path` in the json document.
///
/// don't worry, I ripped this off from myself: https://github.com/hoodie/asciii-rs/blob/master/src/util/yaml.rs
/// literally ported this from yaml to json with regex
///
fn get_path<'a>(json:&'a JsonValue, path:&[&str]) -> Option<&'a JsonValue>{
    if let Some((&key, remainder)) = path.split_first(){

        return match *json{
            JsonValue::Object(ref hash) =>
            {
                if remainder.is_empty(){
                    hash.get(key)
                } else {
                    hash.get(key)
                        .and_then(|c| get_path(c, remainder))
                }
            },

            JsonValue::Array(ref vec) =>
            {
                if let Ok(index) = key.parse::<usize>() {
                    if remainder.is_empty(){
                        vec.get(index)
                    } else {
                        vec.get(index).and_then(|c| get_path(c, remainder))
                    }
                } else { None }
            },
            _ => None
        }

    }
    None
}

fn main() {
    let client = Client::new();

    let mut res = client.get(&station_url("SLUB")).send().unwrap();
    let mut res_content = String::new();
    res.read_to_string(&mut res_content).unwrap();
    let dvb_response = json::parse(&res_content).unwrap();
    // original example from json crate doc
    let test_json = json::parse(r#" { "code": 200, "success": true, "payload": { "features": [ "awesome", "easyAPI", "lowLearningCurve" ] } } "#).unwrap();


    println!("{}", get(&dvb_response, "1/0/0").and_then(|j|j.as_str()).unwrap());
    println!("this is {:?} shit", get(&test_json, "payload/features/0").unwrap().as_str());

}

