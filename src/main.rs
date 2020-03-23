#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate pulldown_cmark;
extern crate yaml_rust;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io::{BufReader, BufRead};
use std::fs::{File};

use rocket_contrib::templates::Template;
use pulldown_cmark::{Parser, Options, html};
use yaml_rust::{YamlLoader};
use crate::models::{Model, SlugAndDoc, HeaderAndContent};
use std::path::PathBuf;
use rocket_contrib::templates::tera::{self, Value};
use std::collections::HashMap;
use serde_json::json;

mod routes;
mod models;

fn my_filter(value: Value, _: HashMap<String, Value>) -> tera::Result<Value> {
    let mut result = String::new();

    let month = value["month"].as_u64().unwrap();
    let month = match month {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "May",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        11 => "Dec",
        _ => panic!("Invalid month")
    };

    result.push_str(month);
    result.push(' ');
    let day = value["day"].as_u64().unwrap();
    if day < 10 {
        result.push('0');
    }
    result.push_str(&day.to_string());
    result.push_str(", ");
    let year = value["year"].as_u64().unwrap();
    result.push_str(&year.to_string());

    Ok(json!(result))
}

fn main() {
    let fairing = Template::custom(|engines| {
        engines.tera.register_filter("date", my_filter)
    });
    rocket::ignite()
        .mount("/", routes![
            routes::index,
            routes::posts::index,
            routes::posts::get,
            routes::courses::index,
            routes::courses::get,
            routes::courses::get_chapter,
            routes::publications::index,
            routes::publications::get,
            routes::static_files
        ])
        .attach(fairing)
        .launch();
}

fn read_header_and_content<T>(folder: PathBuf) -> Option<HeaderAndContent<T>>
  where T : for <'de> From<SlugAndDoc<'de>> + Model {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let index_file = folder.join("index.md");
    let md = File::open(index_file);
    md.map(|file| {
        let reader = BufReader::new(file);
        let mut header_buf = String::new();
        let mut content_buf = String::new();
        let mut counter = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            if counter >= 2 {
                content_buf.push_str(&line);
                content_buf.push('\n');
            }

            if counter < 2 {
                header_buf.push_str(&line);
                header_buf.push('\n');
            }

            if line.starts_with("---") {
                counter += 1;
            }

        }
        let slug = folder.file_name().and_then(|it| it.to_str()).unwrap().to_string().clone();
        let docs = YamlLoader::load_from_str(&header_buf).unwrap();
        let header = T::from(SlugAndDoc::new(slug, &docs[0]));
        let mut content = String::new();
        let parser = Parser::new_ext(&content_buf, options);
        html::push_html(&mut content, parser);
        HeaderAndContent { header, content }
    }).ok()
}

fn read_header<T>(folder: PathBuf) -> Option<T> where T: for<'de> From<SlugAndDoc<'de>> + Model {
    let index_file = folder.join("index.md");
    let md = File::open(index_file);
    md.map(|file| {
        let reader = BufReader::new(file);
        let mut content = String::new();
        let mut counter = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("---") {
                counter += 1;
            }
            content.push_str(&line);
            content.push('\n');
            if counter == 2 {
                break;
            }
        }
        let slug = folder.file_name().unwrap().to_str().unwrap().to_string().clone();
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let header = T::from(SlugAndDoc::new(slug, &docs[0]));
        header
    }).ok()
}