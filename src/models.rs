use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use chrono::{DateTime, Local};
use tokio::sync::Mutex;

use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub email: String,
    pub pool: Arc<Mutex<DbPool>>,
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
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
