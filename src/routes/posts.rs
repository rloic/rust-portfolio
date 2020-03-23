use std::path::PathBuf;
use crate::models::HeaderAndContent;
use crate::models::posts::Post;
use rocket_contrib::templates::Template;
use crate::{read_header_and_content, read_header};
use crate::routes::ARTICLES;
use rocket_contrib::templates::tera::Context;
use std::fs::read_dir;

#[get("/posts")]
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

    let mut ctx = Context::new();
    ctx.insert("title", "Articles");
    ctx.insert("localisation", "articles");
    ctx.insert("posts", &posts);
    Template::render("posts/index", &ctx)
}

#[get("/posts/<slug>")]
pub fn get(slug: String) -> Option<Template> {
    let post: Option<HeaderAndContent<Post>> = read_header_and_content(PathBuf::from(ARTICLES).join(slug));

    post.map(|post| {
        let mut ctx = Context::new();
        ctx.insert("title", "Articles");
        ctx.insert("localisation", "articles");
        ctx.insert("post", &post.header);
        ctx.insert("content", &post.content);
        Template::render("posts/get", &ctx)
    })
}