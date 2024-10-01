use std::{fs::File, net::SocketAddr, sync::Arc};

use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use models::AppState;
use pgp::Deserializable;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

mod config;
#[cfg(feature = "postgres")]
#[path = "db_postgres.rs"]
mod db;
#[cfg(not(feature = "postgres"))]
#[path = "db_sqlite.rs"]
mod db;
mod handler;
mod models;
mod templates;
mod verify;

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
    let public_key = if let Some(public_key_path) = config.public_key {
        let (key, _) = pgp::SignedPublicKey::from_armor_single(File::open(public_key_path)?)?;
        Some(key)
    } else {
        None
    };
    let pool = db::create_pool(&config.db_url).await?;
    let state = AppState {
        name: config.author.name,
        email: config.author.email,
        tz: config.timezone,
        public_key,
        pool: Arc::new(Mutex::new(pool)),
    };

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/paginated") }))
        .route("/all", get(handler::all_posts))
        .route("/paginated", get(handler::paginated_posts))
        .route("/date/:date", get(handler::date_posts))
        .route("/search", get(handler::search_result))
        .route("/post/:id", get(handler::single_post))
        .route("/post/:id/raw", get(handler::single_post_raw))
        .route("/add_post", get(handler::add_post_form))
        .route("/preview", get(handler::preview_post))
        .route("/api/posts", get(handler::fetch_grouped_posts))
        .route("/api/posts/:date", get(handler::fetch_date_posts))
        .route("/api/search", get(handler::fetch_search_result))
        .route("/api/new_post", post(handler::new_post))
        .route("/teapot", get(handler::teapot))
        .fallback(get(handler::teapot))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(SocketAddr::new(config.host, config.port)).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
