use crate::{
    domain::{Status, Task},
    repo::Repo,
    Error, Result,
};
use tokio_postgres::Row;

use crate::db::sql;

/// Row mapper for the task domain object.
impl From<&Row> for Task {
    fn from(row: &Row) -> Self {
        Task::new(row.get(0), row.get(1), row.get(2), row.get(3))
    }
}

impl Repo {
    /// Select a task by id
    pub async fn select_task(&self, id: i32) -> Result<Task> {
        tracing::debug!("select_task: {}", id);

        let mut conn = self.pool.get().await?;
        let select_task = conn.prepare_statement(sql::tasks::FETCH).await?;
        let result = conn.query_one(&select_task, &[&id]).await;

        if let Ok(row) = result {
            Ok(Task::from(&row))
        } else {
            Err(Error::not_found(format!("task not found: {}", id)))
        }
    }

    /// Select a page of tasks for a story.
    pub async fn select_tasks(&self, story_id: i32) -> Result<Vec<Task>> {
        tracing::debug!("select_tasks");

        let mut conn = self.pool.get().await?;
        let select_tasks = conn.prepare_statement(sql::tasks::SELECT).await?;

        let tasks: Vec<_> = conn
            .query(&select_tasks, &[&story_id])
            .await?
            .iter()
            .map(Task::from)
            .collect();

        Ok(tasks)
    }

    /// Insert a new task
    pub async fn insert_task(&self, story_id: i32, name: String) -> Result<Task> {
        tracing::debug!("insert_task: {}", name);

        let mut conn = self.pool.get().await?;
        let insert_task = conn.prepare_statement(sql::tasks::INSERT).await?;

        let row = conn.query_one(&insert_task, &[&story_id, &name]).await?;
        let task = Task::new(row.get(0), story_id, name, row.get(1));

        Ok(task)
    }

    /// Delete a task.
    pub async fn delete_task(&self, id: i32) -> Result<u64> {
        tracing::debug!("delete_task: {}", id);

        let mut conn = self.pool.get().await?;
        let delete_task = conn.prepare_statement(sql::tasks::DELETE).await?;

        conn.execute(&delete_task, &[&id])
            .await
            .map_err(Error::from)
    }

    /// Update task name and status.
    pub async fn update_task(&self, id: i32, name: String, status: Status) -> Result<Task> {
        tracing::debug!("update_task: {}, {}, {:?}", id, name, status);

        let mut conn = self.pool.get().await?;
        let update_task = conn.prepare_statement(sql::tasks::UPDATE).await?;

        let status = String::from(status);
        let row = conn.query_one(&update_task, &[&name, &status, &id]).await?;
        let task = Task::new(id, row.get(0), name, status);

        Ok(task)
    }
}
