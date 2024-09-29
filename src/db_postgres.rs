use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;

use crate::models::Post;

pub type DbPool = Pool<sqlx::Postgres>;

pub async fn create_pool(db_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}

pub async fn add_post(pool: &DbPool, sentence: &str) -> Result<i64, sqlx::Error> {
    let result: i64 = sqlx::query_scalar(
        "INSERT INTO posts (sentence) VALUES ($1) RETURNING id",
    )
    .bind(sentence)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_all_posts(pool: &DbPool) -> Vec<Post> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY timestamp DESC")
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts.")
}

pub async fn get_posts_by_page(pool: &DbPool, page: i64, posts_per_page: i64) -> Vec<Post> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY timestamp DESC LIMIT $1 OFFSET $2")
        .bind(posts_per_page)
        .bind((page - 1) * posts_per_page)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts.")
}

pub async fn get_posts_by_date(pool: &DbPool, date: &str) -> Vec<Post> {
    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Protocol(e.to_string())).unwrap();
    sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE DATE(timestamp AT TIME ZONE $2) = $1 ORDER BY timestamp DESC",
    )
    .bind(parsed_date)
    .bind("Asia/Shanghai")
    .fetch_all(pool)
    .await
    .expect("Failed to fetch posts.")
}

pub async fn get_post_by_id(pool: &DbPool, id: i64) -> Option<Post> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .expect("Failed to fetch post.")
}

pub async fn get_post_count(pool: &DbPool) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM posts")
        .fetch_one(pool)
        .await
        .expect("Failed to fetch post count.")
}