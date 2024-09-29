use askama::Template;

use crate::models::Post;


#[derive(Template)]
#[template(path = "all_posts.html")]
pub struct AllPostsTemplate {
    pub name: String,
    pub email: String,
}

#[derive(Template)]
#[template(path = "add_post.html")]
pub struct AddPostTemplate {
    pub error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "teapot.html")]
pub struct TeapotTemplate;

#[derive(Template)]
#[template(path = "post.html")]
pub struct SinglePostTemplate {
    pub name: String,
    pub email: String,
    pub post: Option<Post>,
}