use yaml_rust::Yaml;
use std::collections::HashMap;


#[derive(Clone, Debug, Serialize)]
pub struct Resource {
    name: String,
    icon: Option<String>,
    url: String,
}

impl<'l> From<&'l Yaml> for Resource {
    fn from(doc: &'l Yaml) -> Self {
        let mut mapping = HashMap::new();
        mapping.insert("pdf", "far fa-file-pdf");
        mapping.insert("time", "fas fa-stopwatch");
        mapping.insert("code", "fas fa-code");
        mapping.insert("url", "fas fa-link");
        mapping.insert("link", "fas fa-link");
        mapping.insert("chart", "far fa-chart-bar");
        mapping.insert("cogs", "fas fa-cogs");
        mapping.insert("cogs", "fas fa-cogs");
        mapping.insert("bookmark", "fas fa-bookmark");
        mapping.insert("build", "fas fa-hammer");

        let name = doc["name"].as_str().unwrap().to_owned();
        let icon = doc["icon"].as_str().and_then(|kind| mapping.get(&kind).map(|it| (*it).to_owned()));
        let url = doc["url"].as_str().unwrap().to_owned();
        Resource { name, icon, url }
    }
}