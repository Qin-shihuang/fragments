use std::{net::SocketAddr, sync::Arc};

use axum::{response::Redirect, routing::{get, post}, Router};
use models::AppState;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

mod config;
mod db;
mod handler;
mod models;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let config_path = if args.len() == 2 {
        &args[1]
    } else if args.len() == 1 {
        "config.json"
    } else {
        eprintln!("Usage: {} [config.json]", args[0]);
        std::process::exit(1);
    };
    let config = config::Config::new(config_path)?;

    let pool = db::create_pool(&config.db_url).await?;
    db::init_schema(&pool).await?; 
    let state = AppState {
        name: config.author.name,
        email: config.author.email,
        pool: Arc::new(Mutex::new(pool)),
    };

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/paginated") }))
        .route("/paginated", get(handler::paginated_posts))
        .route("/all", get(handler::all_posts))
        .route("/post/:id", get(handler::single_post))
        .route("/add_post", get(handler::add_post_form))
        .route("/api/posts", get(handler::fetch_grouped_posts))
        .route("/api/new_post", post(handler::new_post))
        
        .fallback(get(handler::teapot))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(SocketAddr::new(config.host, config.port))
        .await?;
    axum::serve(listener, app.into_make_service())
        .await?;
    Ok(())
}
