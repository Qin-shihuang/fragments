use askama::Template;

use crate::models::Post;

#[derive(Template)]
#[template(path = "all_posts.html")]
pub struct AllPostsTemplate {
    pub name: String,
    pub email: String,
}

#[derive(Template)]
#[template(path = "paginated_posts.html")]
pub struct PaginatedPostsTemplate {
    pub name: String,
    pub email: String,
    pub current_page: i64,
    pub total_pages: i64,
    pub per_page: i64,
}

#[derive(Template)]
#[template(path = "date_posts.html")]
pub struct DatePostsTemplate {
    pub name: String,
    pub email: String,
}

#[derive(Template)]
#[template(path = "search_result.html")]
pub struct SearchResultTemplate {
    pub name: String,
    pub email: String,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct SinglePostTemplate {
    pub name: String,
    pub email: String,
    pub tz: String,
    pub post: Option<Post>,
}

#[derive(Template)]
#[template(path = "add_post.html")]
pub struct AddPostTemplate {
    pub error_message: Option<String>,
    pub keyid: Option<String>,
}

#[derive(Template)]
#[template(path = "preview_post.html")]
pub struct PreviewPostTemplate {
    pub name: String,
    pub email: String,
    pub is_from_add: bool,
}

#[derive(Template)]
#[template(path = "teapot.html")]
pub struct TeapotTemplate;
