use std::path::PathBuf;
use rocket_contrib::templates::Template;
use crate::{read_header, read_header_and_content};
use crate::routes::PUBLICATIONS;
use rocket_contrib::templates::tera::Context;
use std::fs::read_dir;
use crate::models::publications::Publication;
use crate::models::HeaderAndContent;

#[get("/publications")]
pub fn index() -> Template {
    let files = read_dir(PathBuf::from(PUBLICATIONS)).ok();

    let mut publications = Vec::new();
    files.map(|it| {
        for path in it {
            let folder = path.unwrap();
            let header: Option<Publication> = read_header(folder.path());
            if let Some(header) = header {
                publications.push(header);
            }
        }
    });
    publications.sort_by(|lhs, rhs| lhs.publication_date.cmp(&rhs.publication_date).reverse());

    let mut ctx = Context::new();
    ctx.insert("title", "Publications");
    ctx.insert("localisation", "publications");
    ctx.insert("publications", &publications);
    Template::render("publications/index", &ctx)
}

#[get("/publications/<slug>")]
pub fn get(slug: String) -> Option<Template> {
    let publication: Option<HeaderAndContent<Publication>> = read_header_and_content(PathBuf::from(PUBLICATIONS).join(slug));

    publication.map(|publication| {
        let mut ctx = Context::new();
        ctx.insert("title", "Articles");
        ctx.insert("localisation", "articles");
        ctx.insert("publication", &publication.header);
        ctx.insert("content", &publication.content);
        Template::render("publications/get", &ctx)
    })
}