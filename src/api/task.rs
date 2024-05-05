use crate::{
    api::{
        dto::{CreateTaskBody, PatchTaskBody},
        Ctx,
    },
    Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;

/// API routes for tasks
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new().route("/tasks", post(create_task)).route(
        "/tasks/:id",
        get(get_task).delete(delete_task).patch(update_task),
    )
}

/// Get a task by id
async fn get_task(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /tasks/{}", id);
    let task = ctx.repo.select_task(id).await?;
    Ok(Json(task))
}

/// Create a new task
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<CreateTaskBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /tasks");
    tracing::debug!("body = {:?}", body);

    let (story_id, name) = body.validate()?;
    let task = ctx
        .repo
        .select_story(story_id)
        .and_then(|story| ctx.repo.insert_task(story.id, name))
        .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// Delete a task by id
async fn delete_task(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /tasks/{}", id);
    match ctx.repo.delete_task(id).await {
        Err(err) => err.into(),
        Ok(num_rows) => {
            if num_rows > 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        }
    }
}

/// Update a task.
async fn update_task(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<PatchTaskBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("PATCH /tasks/{}", id);
    tracing::debug!("body = {:?}", body);
    let existing_task = ctx.repo.select_task(id).await?;
    let (name, status) = body.validate(existing_task)?;
    let updated_task = ctx.repo.update_task(id, name, status).await?;
    Ok(Json(updated_task))
}
