use crate::{
    api::dto::{Page, PagingParams, StoryBody},
    api::Ctx,
    Result,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures::TryFutureExt;
use std::sync::Arc;

/// API routes for stories
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id/tasks", get(get_tasks))
        .route(
            "/stories/:id",
            get(get_story).delete(delete_story).patch(update_story),
        )
}

/// Get a story by id
async fn get_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}", id);
    let story = ctx.repo.select_story(id).await?;
    Ok(Json(story))
}

/// Get tasks for a story
async fn get_tasks(
    params: Option<Query<PagingParams>>,
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}/tasks", id);

    let page_id = params.unwrap_or_default().page_id.unwrap_or(1);
    tracing::debug!("page_id = {}", page_id);

    let data = ctx.repo.select_tasks(id, page_id).await?;
    let next: i32 = data.iter().map(|t| t.id).max().unwrap_or_default() + 1;
    let page = Page::new(1, next, data);

    Ok(Json(page))
}

/// Get a page of stories
async fn get_stories(
    params: Option<Query<PagingParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories");

    let page_id = params.unwrap_or_default().page_id.unwrap_or(1);
    tracing::debug!("page_id = {}", page_id);

    let (prev, next, data) = ctx.repo.select_stories_page(page_id).await?;
    let page = Page::new(prev, next, data);

    Ok(Json(page))
}

/// Create a new story
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /stories");
    tracing::debug!("body = {:?}", body);
    let name = body.validate()?;
    let story = ctx.repo.insert_story(name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Delete a story by id
async fn delete_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /stories/{}", id);
    if let Ok(num_rows) = ctx.repo.delete_story(id).await {
        if num_rows > 0 {
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}

/// Update an existing story
async fn update_story(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("PATCH /stories/{}", id);
    tracing::debug!("body = {:?}", body);

    let name = body.validate()?;
    let story = ctx
        .repo
        .select_story(id)
        .and_then(|_| ctx.repo.update_story(id, name))
        .await?;

    Ok(Json(story))
}
