use crate::models::{Model, str_vec, date, SlugAndDoc, optional_vec};
use crate::models::date::Date;
use crate::models::resources::Resource;

#[derive(Debug, Serialize, Clone)]
pub struct Publication {
    pub slug: String,
    title: String,
    authors: Vec<String>,
    pub publication_date: Date,
    resources: Vec<Resource>
}

impl Model for Publication {
    fn slug(&self) -> String {
        self.slug.clone()
    }

    fn set_slug(&mut self, slug: String) {
        self.slug = slug
    }
}

impl <'l> From<SlugAndDoc<'l>> for Publication {
    fn from(slug_and_doc: SlugAndDoc<'l>) -> Self {
        let slug = slug_and_doc.slug;
        let doc = slug_and_doc.doc;
        let title = doc["title"].as_str().unwrap().to_string();
        let authors = str_vec(doc, "authors");
        let publication_date = date(doc, "publication_date");
        let resources = optional_vec(doc, "resources").unwrap_or(vec![]);
        Publication { slug, title, authors, publication_date, resources }
    }
}