use crate::{
    api::{
        dto::{StoryBody, StoryDto},
        Ctx,
    },
    Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures::{future::try_join, TryFutureExt};
use std::sync::Arc;

/// API routes for stories
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route(
            "/stories/:id",
            get(get_story).delete(delete_story).patch(update_story),
        )
}

/// Get a story by id
async fn get_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}", id);
    let (story, tasks) = try_join(ctx.repo.select_story(id), ctx.repo.select_tasks(id)).await?;
    let dto = StoryDto {
        id: story.id,
        name: story.name,
        tasks,
    };
    Ok(Json(dto))
}

/// Get a page of stories
async fn get_stories(State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories");
    let stories = ctx.repo.select_stories().await?;
    Ok(Json(stories))
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
