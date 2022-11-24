#![allow(dead_code)]

use std::{fs::File, io::Read};

use quickxml_to_serde::{xml_string_to_json, Config};

mod config;
mod db;
mod filter;
mod model;
mod shell;

fn main() {
    xml2json();
    println!("Hello, world!");
}

fn xml2json() {
    // read an XML file into a string
    let mut xml_file = File::open("data/full_database.xml").unwrap();
    let mut xml_contents = String::new();
    println!("read_to_string");

    xml_file.read_to_string(&mut xml_contents).unwrap();

    println!("xml_string_to_json");

    // convert the XML string into JSON with default config params
    let json = xml_string_to_json(xml_contents, &Config::new_with_defaults()).unwrap();

    let d = json
        .as_object()
        .unwrap()
        .get("drugbank")
        .unwrap()
        .as_object()
        .unwrap()
        .get("drug")
        .unwrap()
        .as_array()
        .unwrap();

    d.iter().enumerate().for_each(|f| {
        println!("write to file {} ", f.0);

        serde_json::to_writer(
            &File::create(format!("data/json/{}.json", f.0)).unwrap(),
            &f.1,
        )
        .unwrap();
    });

    // serde_json::to_writer(&File::create("data/full.json").unwrap(), &json).unwrap();

    // println!("{}", json);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_xml2json() {
        xml2json();
    }
}
