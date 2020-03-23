use std::path::PathBuf;
use rocket_contrib::templates::Template;
use crate::{read_header, read_header_and_content};
use crate::routes::COURSES;
use rocket_contrib::templates::tera::Context;
use std::fs::{read_dir, metadata};
use crate::models::courses::{Course, Chapter};
use crate::models::HeaderAndContent;
use std::collections::HashMap;

fn load_chapters(slug: String) -> Vec<Chapter> {
    let mut chapters = Vec::new();
    let parts = read_dir(PathBuf::from(COURSES).join(slug)).ok();
    parts.map(|it| {
        for path in it {
            let path = path.unwrap();
            if metadata(path.path()).unwrap().is_dir() {
                let chapter: Option<Chapter>
                    = read_header(path.path());
                if let Some(chapter) = chapter {
                    chapters.push(chapter);
                }
            }
        }
    });
    chapters.sort_by_key(|it| it.slug.clone());
    chapters
}

#[get("/courses")]
pub fn index() -> Template {
    let files = read_dir(PathBuf::from(COURSES)).ok();

    let mut categories: HashMap<String, Vec<Course>> = HashMap::new();
    files.map(|it| {
        for path in it {
            let folder = path.unwrap();
            let header: Option<Course> = read_header(folder.path());
            if let Some(header) = header {
                let entry = categories.entry(header.subject.clone()).or_insert(Vec::new());
                entry.push(header);
            }
        }
    });

    for entry in categories.iter_mut() {
        entry.1.sort_by(|lhs, rhs| lhs.publication_date.cmp(&rhs.publication_date).reverse());
    }

    let mut ctx = Context::new();
    ctx.insert("title", "Courses");
    ctx.insert("localisation", "courses");
    ctx.insert("course_categories", &categories);
    Template::render("courses/index", &ctx)
}

#[get("/courses/<slug>")]
pub fn get(slug: String) -> Option<Template> {
    let header_and_content: Option<HeaderAndContent<Course>>
        = read_header_and_content(PathBuf::from(COURSES).join(slug.clone()));

    header_and_content.map(|course| {
        let chapters = load_chapters(slug);
        let mut ctx = Context::new();
        ctx.insert("title", "Articles");
        ctx.insert("localisation", "courses");
        ctx.insert("course", &course.header);
        ctx.insert("content", &course.content);
        ctx.insert("chapters", &chapters);
        Template::render("courses/get", &ctx)
    })
}

#[get("/courses/<slug>/<chapter>")]
pub fn get_chapter(slug: String, chapter: String) -> Option<Template> {
    let header_and_content: Option<Course>
        = read_header(PathBuf::from(COURSES).join(slug.clone()));

    header_and_content.and_then(|course| {
        let chapter: Option<HeaderAndContent<Chapter>>
          = read_header_and_content(PathBuf::from(COURSES).join(slug.clone()).join(chapter));
        chapter.map(|it| (course, it))
    }).map(|(course, chapter)| {

        let chapters = load_chapters(slug);

        let mut ctx = Context::new();
        ctx.insert("title", "Articles");
        ctx.insert("localisation", "courses");
        ctx.insert("course", &course);
        ctx.insert("chapter", &chapter.header);
        ctx.insert("content", &chapter.content);
        ctx.insert("chapters", &chapters);
        ctx.insert("content", &chapter.content);
        Template::render("courses/get_chapter", &ctx)
    })
}