use crate::models::{Model, optional_str_vec, str_vec, date, SlugAndDoc};
use crate::models::date::Date;

#[derive(Debug, Serialize, Clone)]
pub struct Post {
    pub slug: String,
    title: String,
    summary: String,
    long_summary: String,
    authors: Vec<String>,
    tags: Vec<String>,
    categories: Vec<String>,
    featured: bool,
    draft: bool,
    projects: Vec<String>,
    icon: Option<String>,
    pub publication_date: Date,
}

impl Post {
    pub fn new(
        slug: String,
        title: String,
        summary: String,
        long_summary: String,
        authors: Vec<String>,
        tags: Vec<String>,
        categories: Vec<String>,
        featured: bool,
        draft: bool,
        projects: Vec<String>,
        icon: Option<String>,
        publication_date: Date,
    ) -> Post {
        Post { slug, title, summary, long_summary, authors, tags, categories, featured, draft, projects, icon, publication_date }
    }
}

impl Model for Post {
    fn slug(&self) -> String {
        self.slug.clone()
    }

    fn set_slug(&mut self, slug: String) {
        self.slug = slug;
    }
}

impl<'l> From<SlugAndDoc<'l>> for Post {
    fn from(slug_and_doc: SlugAndDoc) -> Self {
        let slug = slug_and_doc.slug;
        let doc = slug_and_doc.doc;
        let title = doc["title"].as_str().unwrap().to_string();
        let summary = doc["summary"].as_str().unwrap_or("").to_string();
        let long_summary = doc["abstract"].as_str().unwrap_or("").to_string();
        let authors = str_vec(doc, "authors");
        let tags = optional_str_vec(doc, "tags");
        let categories = optional_str_vec(doc, "categories");
        let featured = doc["featured"].as_bool().unwrap_or(false);
        let draft = doc["draft"].as_bool().unwrap_or(false);
        let projects = optional_str_vec(doc, "projects");
        let icon = doc["icon"].as_str().map(|it| it.to_string());
        let publication_date = date(doc, "publication_date");

        Post::new(slug, title, summary, long_summary, authors, tags, categories, featured, draft, projects, icon, publication_date)
    }
}