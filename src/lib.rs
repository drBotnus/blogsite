use frontmatter::FrontMatter;

mod frontmatter;
mod routes;

struct AppState {
    posts: Vec<FrontMatter>,
}
