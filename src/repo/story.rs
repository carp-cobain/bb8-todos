use crate::{domain::Story, repo::Repo, Error, Result};
use futures::StreamExt;
use tokio::pin;

use crate::db::sql;

const PAGE_SIZE: usize = 100;

impl Repo {
    /// Select a story by id
    pub async fn select_story(&self, id: i32) -> Result<Story> {
        tracing::debug!("select_story: {}", id);

        let mut conn = self.pool.get().await?;
        let select_story = conn.prepare_cache(sql::stories::FETCH).await?;

        let stream = conn.query_raw(&select_story, &[&id]).await?;
        pin!(stream);

        if let Some(result) = stream.next().await {
            let row = result?;
            Ok(Story::new(row.get(0), row.get(1)))
        } else {
            Err(Error::not_found(format!("story not found: {}", id)))
        }
    }

    /// Select a page of stories with previous and next page cursors.
    pub async fn select_stories(&self, page_id: i32) -> Result<(i32, i32, Vec<Story>)> {
        tracing::debug!("select_stories");

        let mut conn = self.pool.get().await?;
        let select_stories = conn.prepare_cache(sql::stories::SELECT).await?;

        let stream = conn.query_raw(&select_stories, &[page_id]).await?;
        pin!(stream);

        let mut prev_pid: i32 = 0;
        let mut next_pid: i32 = 0;
        let mut stories = Vec::with_capacity(PAGE_SIZE);

        while let Some(result) = stream.next().await {
            let row = result?;
            let label: &str = row.get(2);

            if label == "current" {
                stories.push(Story::new(row.get(0), row.get(1)));
            } else if label == "prev" {
                prev_pid = row.get(0);
            } else if label == "next" {
                next_pid = row.get(0);
            } else {
                tracing::warn!("unknown page label: {}", label);
            }
        }

        Ok((prev_pid, next_pid, stories))
    }

    /// Insert a new story
    pub async fn insert_story(&self, name: String) -> Result<Story> {
        tracing::debug!("insert_story: {}", name);

        let mut conn = self.pool.get().await?;
        let insert_story = conn.prepare_cache(sql::stories::INSERT).await?;

        let stream = conn.query_raw(&insert_story, &[&name]).await?;
        pin!(stream);

        if let Some(result) = stream.next().await {
            let row = result?;
            Ok(Story::new(row.get(0), name))
        } else {
            Err(Error::internal(format!("failed to insert story: {}", name)))
        }
    }

    /// Delete a story.
    pub async fn delete_story(&self, id: i32) -> Result<u64> {
        tracing::debug!("delete_story: {}", id);

        let mut conn = self.pool.get().await?;
        let delete_tasks = conn.prepare_cache(sql::tasks::DELETE_BY_STORY).await?;
        let delete_story = conn.prepare_cache(sql::stories::DELETE).await?;

        let tx = conn.transaction().await?;

        // Delete all tasks for the story
        let num_tasks = tx
            .execute_raw(&delete_tasks, &[&id])
            .await
            .map_err(Error::from)?;

        // Delete the story
        let num_stories = tx
            .execute_raw(&delete_story, &[&id])
            .await
            .map_err(Error::from)?;

        tx.commit().await?;

        Ok(num_tasks + num_stories)
    }

    /// Update a story.
    pub async fn update_story(&self, id: i32, name: String) -> Result<Story> {
        tracing::debug!("update_story: {}, {}", id, name);

        let mut conn = self.pool.get().await?;
        let update_story = conn.prepare_cache(sql::stories::UPDATE).await?;

        let num_rows = conn.execute(&update_story, &[&name, &id]).await?;
        if num_rows > 0 {
            Ok(Story::new(id, name))
        } else {
            Err(Error::internal("unable to update story".into()))
        }
    }
}
