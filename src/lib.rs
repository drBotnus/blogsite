use std::fs;

use axum::{routing::get, Router};
use frontmatter::FrontMatter;

mod frontmatter;
mod routes;

pub struct AppState {
    posts: Vec<FrontMatter>,
}

pub fn startup() -> Result<Router, String> {
    let posts = match fs::read_dir("content/blog") {
        Ok(files) => {
            let mut posts = files
                .into_iter()
                .filter_map(|file| file.ok())
                .filter_map(|file| fs::read_to_string(file.path()).ok())
                .filter_map(|file| FrontMatter::from_file(file).ok())
                .filter(|frontmatter| !frontmatter.draft.unwrap_or(false))
                .collect::<Vec<_>>();
            posts.sort_by(|a, b| b.date.cmp(&a.date));
            Ok(posts)
        }
    }?;

    let state = AppState { posts };

    let blog_routes = Router::new()
        .route("/", get(routes::blog))
        .route("/{post_name}", get(routes::get_blog_post));
}
