use crate::{
    api::dto::StoryBody,
    api::paging::{Page, PageParams, PageToken},
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
    params: Option<Query<PageParams>>,
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}/tasks", id);

    // Determine page to query
    let q = params.unwrap_or_default();
    let page_id = PageToken::decode(q.page_token.clone())?;

    // Query and create page
    let data = ctx.repo.select_tasks(id, page_id).await?;
    let next = data.last().and_then(|t| PageToken::encode(t.id + 1));
    let page = Page::new(None, next, data);

    Ok(Json(page))
}

/// Get a page of stories
async fn get_stories(
    params: Option<Query<PageParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories");

    // Determine page to query
    let q = params.unwrap_or_default();
    let page_id = PageToken::decode(q.page_token.clone())?;

    // Query and create page
    let (prev, next, data) = ctx.repo.select_stories(page_id).await?;
    let page = Page::new(PageToken::encode(prev), PageToken::encode(next), data);

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
