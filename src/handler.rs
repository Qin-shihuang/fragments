use crate::db::{add_post, get_post_by_id, get_post_count, get_posts, get_posts_by_page};
use crate::models::{AppState, GroupedPosts, PaginationParams, PostForm};
use crate::templates::{
    AddPostTemplate, AllPostsTemplate, PaginatedPostsTemplate, SinglePostTemplate, TeapotTemplate,
};

use axum::extract::{Form, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Json;
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
) -> Result<PaginatedPostsTemplate, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let pool = state.pool.lock().await;
    let total_posts = get_post_count(&pool).await;

    let total_pages = (total_posts as f64 / per_page as f64).ceil() as u32;
    Ok(PaginatedPostsTemplate {
        name: state.name,
        email: state.email,
        current_page: page,
        total_pages: total_pages as i64,
        per_page,
    })
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
            post,
        },
    )
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
        get_posts(&pool).await
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

pub async fn new_post(
    State(state): State<AppState>,
    Form(input): Form<PostForm>,
) -> impl IntoResponse {
    let pool = state.pool.lock().await;
    match add_post(&pool, &input.sentence).await {
        Ok(id) => Redirect::to(&format!("/post/{}", id)).into_response(),
        Err(e) => {
            let error_message = format!("Failed to add post: {}", e);
            AddPostTemplate {
                error_message: Some(error_message),
            }
            .into_response()
        }
    }
}

pub async fn add_post_form() -> impl IntoResponse {
    AddPostTemplate {
        error_message: None,
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
