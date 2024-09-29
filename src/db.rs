use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::models::Post;

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool(db_url: &str) -> Result<DbPool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}

pub async fn init_schema(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            sentence TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn add_post(pool: &SqlitePool, sentence: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO posts (sentence) VALUES (?)")
        .bind(sentence)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_posts(pool: &SqlitePool) -> Vec<Post> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY timestamp DESC")
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts.")
}

pub async fn get_posts_by_page(pool: &SqlitePool, page: i64, posts_per_page: i64) -> Vec<Post> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY timestamp DESC LIMIT ? OFFSET ?")
        .bind(posts_per_page)
        .bind((page - 1) * posts_per_page)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch posts.")
}

pub async fn get_posts_by_date(pool: &SqlitePool, date: &str) -> Vec<Post> {
    sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE date(timestamp) = ? ORDER BY timestamp DESC",
    )
    .bind(date)
    .fetch_all(pool)
    .await
    .expect("Failed to fetch posts.")
}

pub async fn get_post_by_id(pool: &SqlitePool, id: i64) -> Option<Post> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .expect("Failed to fetch post.")
}
