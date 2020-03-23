use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs::read_dir;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use crate::read_header;
use crate::models::posts::Post;
use crate::models::publications::Publication;
use crate::models::courses::Course;

pub mod courses;
pub mod posts;
pub mod publications;

const ARTICLES: &str = "content/posts";
const COURSES: &str = "content/courses";
const PUBLICATIONS: &str = "content/publications";
const PROJECTS: &str = "content/projects";

#[get("/")]
pub fn index() -> Template {
    let files = read_dir(PathBuf::from(ARTICLES)).ok();

    let mut posts = Vec::new();
    files.map(|it| {
        for path in it {
            let folder = path.unwrap();
            let header: Option<Post> = read_header(folder.path());
            if let Some(header) = header {
                posts.push(header);
            }
        }
    });
    posts.sort_by(|lhs, rhs| lhs.publication_date.cmp(&rhs.publication_date).reverse());

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

    let files = read_dir(PathBuf::from(COURSES)).ok();

    let mut course_categories = HashMap::<String, Vec<Course>>::new();
    files.map(|it| {
        for path in it {
            let folder = path.unwrap();
            let header: Option<Course> = read_header(folder.path());
            if let Some(header) = header {
                let entry = course_categories.entry(header.subject.clone()).or_insert(Vec::new());
                entry.push(header);
            }
        }
    });
    // course_categories.sort_by(|lhs, rhs| lhs.publication_date.cmp(&rhs.publication_date));

    for entry in course_categories.iter_mut() {
        entry.1.sort_by(|lhs, rhs| lhs.publication_date.cmp(&rhs.publication_date).reverse());
        entry.1.truncate(3);
    }

    let len = course_categories.len();

    let mut ctx = Context::new();
    ctx.insert("title", "Loïc Rouquette - Home");
    ctx.insert("localisation", "home");
    ctx.insert("last_post", &posts[0]);
    let res = posts.iter().by_ref().skip(1).take(3).collect::<Vec<_>>();
    ctx.insert("posts", &res);
    ctx.insert("course_categories", &course_categories);
    ctx.insert("projects", &Vec::<u8>::new());
    ctx.insert("publications", &publications);
    Template::render("index", &ctx)
}

#[get("/<file..>", rank = 999)]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
    let mut redirect = HashMap::<&str, &str>::new();
    redirect.insert("css", "static/style/");
    redirect.insert("js", "static/script/");

    let ext = file.extension().map(|ext| ext.to_str().unwrap());
    ext.and_then(|valid_ext| {
        let path = redirect.get(valid_ext);
        path.and_then(|valid_path| {
            NamedFile::open(Path::new(*valid_path).join(file.clone())).ok()
        }).or(NamedFile::open(Path::new("static/assets").join(file.clone())).ok())
    })
}