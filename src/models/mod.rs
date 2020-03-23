pub mod date;
pub mod posts;
pub mod publications;
pub mod courses;
pub mod resources;

use yaml_rust::Yaml;
use crate::models::date::Date;

pub struct SlugAndDoc<'l> {
    pub slug: String,
    pub doc: &'l Yaml
}

impl <'l> SlugAndDoc<'l> {
    pub fn new(slug: String, doc: &'l Yaml) -> SlugAndDoc {
        SlugAndDoc { slug, doc }
    }
}

fn date(doc: &Yaml, field: &str) -> Date {
    let field = doc[field].as_str().unwrap();
    let parts = field.split("-").collect::<Vec<_>>();

    let day: u8 = parts[2].parse().unwrap();
    let month: u8 = parts[1].parse::<u8>().unwrap() - 1u8;
    let year: u16 = parts[0].parse().unwrap();

    Date { day, month, year }
}

fn str_vec(doc: &Yaml, field: &str) -> Vec<String> {
    let field = doc[field].as_vec().unwrap();
    let content = field.iter()
        .map(|it| it.as_str().unwrap().to_string())
        .collect();
    content
}

fn optional_str_vec(doc: &Yaml, field: &str) -> Vec<String> {
    let opt_field = doc[field].as_vec();
    let opt_content = opt_field.map(|array| {
        array.iter().map(|it| it.as_str().unwrap().to_string()).collect()
    });
    opt_content.unwrap_or(vec![])
}

fn optional_vec<T>(doc: &Yaml, field: &str) -> Option<Vec<T>> where T : for <'l> From<&'l Yaml> {
    doc[field].as_vec().map(|array| {
        array.iter()
            .map(|it| T::from(it))
            .collect()
    })
}

pub trait Model {
    fn slug(&self) -> String;
    fn set_slug(&mut self, slug: String);
}

pub struct HeaderAndContent<T> {
    pub header: T,
    pub content: String
}