use crate::db::{
    add_post, get_all_posts, get_post_by_id, get_post_count, get_posts_by_date, get_posts_by_page,
    search_posts,
};
use crate::models::{AppState, GroupedPosts, PaginationParams, Post, PostForm, PreviewParams, SearchParams};
use crate::templates::{
    AddPostTemplate, AllPostsTemplate, DatePostsTemplate, PaginatedPostsTemplate,
    PreviewPostTemplate, SearchResultTemplate, SinglePostTemplate, TeapotTemplate,
};
use crate::verify::{get_keyid_string, verify_signature};

use axum::extract::{Form, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::collections::BTreeMap;

pub async fn all_posts(State(state): State<AppState>) -> impl IntoResponse {
    AllPostsTemplate {
        name: state.name,
        email: state.email,
    }
}

pub async fn paginated_posts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> PaginatedPostsTemplate {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let pool = state.pool.lock().await;
    let total_posts = get_post_count(&pool).await;

    let total_pages = (total_posts as f64 / per_page as f64).ceil() as u32;
    PaginatedPostsTemplate {
        name: state.name,
        email: state.email,
        current_page: page,
        total_pages: total_pages as i64,
        per_page,
    }
}

pub async fn date_posts(State(state): State<AppState>, _: Path<String>) -> DatePostsTemplate {
    DatePostsTemplate {
        name: state.name,
        email: state.email,
    }
}

pub async fn search_result(
    State(state): State<AppState>,
    _: Option<Query<SearchParams>>,
) -> SearchResultTemplate {
    SearchResultTemplate {
        name: state.name,
        email: state.email,
    }
}

pub async fn single_post(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let pool = state.pool.lock().await;
    let post = get_post_by_id(&pool, id).await;
    let status = if post.is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };
    (
        status,
        SinglePostTemplate {
            name: state.name,
            email: state.email,
            tz: state.tz,
            post,
        },
    )
}

pub async fn single_post_raw(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let pool = state.pool.lock().await;
    let post = get_post_by_id(&pool, id).await;
    if post.is_none() {
        (
            StatusCode::NOT_FOUND,
            axum::http::HeaderMap::new(),
            "Post not found".to_string(),
        )
    } else {
        (
            StatusCode::OK,
            {
                let mut headers = axum::http::HeaderMap::new();
                headers.insert("Content-Type", "text/plain".parse().unwrap());
                headers
            },
            post.unwrap().sentence,
        )
    }
}
pub async fn fetch_grouped_posts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<GroupedPosts>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(0);

    let pool = state.pool.lock().await;

    let posts = if per_page > 0 {
        get_posts_by_page(&pool, page, per_page).await
    } else {
        get_all_posts(&pool).await
    };

    let mut grouped = BTreeMap::new();
    for post in posts {
        grouped
            .entry(post.timestamp.date_naive())
            .or_insert_with(Vec::new)
            .push(post);
    }

    let result: Vec<GroupedPosts> = grouped
        .into_iter()
        .map(|(date, posts)| GroupedPosts {
            date: date.to_string(),
            posts,
        })
        .rev()
        .collect();

    Json(result)
}

pub async fn fetch_date_posts(
    State(state): State<AppState>,
    Path(date): Path<String>,
) -> Json<Vec<GroupedPosts>> {
    let pool = state.pool.lock().await;
    let date = date.to_string().trim().to_string();
    let posts = get_posts_by_date(&pool, &date, &state.tz).await;
    if posts.is_empty() {
        Json(Vec::new())
    } else {
        Json(vec![GroupedPosts { date, posts }])
    }
}

pub async fn fetch_search_result(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<Post>> {
    let pool = state.pool.lock().await;
    let posts = search_posts(&pool, &params.query).await.unwrap_or_default();
    Json(posts)
}

pub async fn new_post(
    State(state): State<AppState>,
    Json(input): Json<PostForm>,
) -> impl IntoResponse {
    let pool = state.pool.lock().await;
    if let Some(public_key) = &state.public_key {
        if let Some(signature) = &input.signature {
            match verify_signature(public_key, &input.sentence, signature) {
                Ok(_) => {}
                Err(e) => {
                    return Json(
                        json!(
                            {
                                "success": false,
                                "error": e
                            }
                        )
                    )
                }
            }
        } else {
            return Json(
                json!(
                    {
                        "success": false,
                        "error": "Signature is required."
                    }
                )
            )
        }
    }
    match add_post(&pool, &input.sentence, input.show).await {
        Ok(id) => {
            Json(
                json!(
                    {
                        "success": true,
                        "id": id
                    }
                )
            )
        }
        Err(e) => {
            let error_message = format!("Failed to add post: {}", e);
            Json(
                json!(
                    {
                        "success": false,
                        "error": error_message
                    }
                )
            )
        }
    }
}

pub async fn add_post_form(State(state): State<AppState>) -> impl IntoResponse {
    AddPostTemplate {
        keyid: state.public_key.as_ref().map(get_keyid_string),
    }
}

pub async fn preview_post(
    State(state): State<AppState>,
    Form(input): Form<PreviewParams>,
) -> impl IntoResponse {
    PreviewPostTemplate {
        name: state.name,
        email: state.email,
        is_from_add: input.add.unwrap_or(false),
    }
}

pub async fn teapot() -> impl IntoResponse {
    (
        StatusCode::IM_A_TEAPOT,
        {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert("Content-Type", "text/html".parse().unwrap());
            headers
        },
        TeapotTemplate,
    )
}
