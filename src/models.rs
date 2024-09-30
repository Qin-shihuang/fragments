use std::sync::Arc;

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub email: String,
    pub tz: String,
    pub pool: Arc<Mutex<PgPool>>,
}

#[derive(FromRow, Serialize)]
pub struct Post {
    pub id: i64,
    pub sentence: String,
    pub timestamp: DateTime<Local>,
}

#[derive(Serialize)]
pub struct GroupedPosts {
    pub date: String,
    pub posts: Vec<Post>,
}

#[derive(Deserialize)]
pub struct PostForm {
    pub sentence: String,
    pub show: bool,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub query: String,
}