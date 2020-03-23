use crate::models::{Model, date, SlugAndDoc, str_vec};
use crate::models::date::Date;

#[derive(Debug, Serialize, Clone)]
pub struct Course {
    pub slug: String,
    title: String,
    pub subject: String,
    authors: Vec<String>,
    pub publication_date: Date
}

impl Model for Course {
    fn slug(&self) -> String {
        self.slug.clone()
    }

    fn set_slug(&mut self, slug: String) {
        self.slug = slug;
    }
}

impl <'l> From<SlugAndDoc<'l>> for Course {
    fn from(slug_and_doc: SlugAndDoc<'l>) -> Self {
        let slug = slug_and_doc.slug;
        let doc = slug_and_doc.doc;

        let title = doc["title"].as_str().unwrap().to_string();
        let subject = doc["subject"].as_str().unwrap().to_string();
        let authors = str_vec(doc, "authors");
        let publication_date = date(doc, "publication_date");

        Course {
            slug,
            title,
            subject,
            authors,
            publication_date
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Chapter {
    pub slug: String,
    title: String,
}

impl Model for Chapter {
    fn slug(&self) -> String {
        self.slug.clone()
    }

    fn set_slug(&mut self, slug: String) {
        self.slug = slug;
    }
}

impl <'l> From<SlugAndDoc<'l>> for Chapter {
    fn from(slug_and_doc: SlugAndDoc<'l>) -> Self {
        let slug = slug_and_doc.slug;
        let doc = slug_and_doc.doc;
        let title = doc["title"].as_str().unwrap().to_string();
        Chapter { slug, title }
    }
}