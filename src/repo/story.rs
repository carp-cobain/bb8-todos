use crate::{domain::Story, repo::Repo, Error, Result};
use futures::StreamExt;
use tokio::pin;

use crate::db::sql;

impl Repo {
    /// Select a story by id
    pub async fn select_story(&self, id: i32) -> Result<Story> {
        tracing::debug!("select_story: {}", id);

        let mut conn = self.pool.get().await?;
        let select_story = conn.prepare_statement(&sql::stories::FETCH).await?;

        let stream = conn.query_raw(&select_story, &[&id]).await?;
        pin!(stream);

        if let Some(result) = stream.next().await {
            let row = result?;
            Ok(Story::new(row.get(0), row.get(1)))
        } else {
            Err(Error::not_found(format!("story not found: {}", id)))
        }
    }

    /// Select a page of stories
    pub async fn select_stories(&self) -> Result<Vec<Story>> {
        tracing::debug!("select_stories");

        let mut conn = self.pool.get().await?;
        let select_stories = conn.prepare_statement(&sql::stories::SELECT).await?;

        let stream = conn
            .query_raw::<_, _, &[i32; 0]>(&select_stories, &[])
            .await?;
        pin!(stream);

        let mut stories = Vec::with_capacity(10);
        while let Some(result) = stream.next().await {
            let row = result?;
            stories.push(Story::new(row.get(0), row.get(1)));
        }

        Ok(stories)
    }

    /// Insert a new story
    pub async fn insert_story(&self, name: String) -> Result<Story> {
        tracing::debug!("insert_story: {}", name);

        let mut conn = self.pool.get().await?;
        let insert_story = conn.prepare_statement(&sql::stories::INSERT).await?;

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
        let delete_story = conn.prepare_statement(&sql::stories::DELETE).await?;

        conn.execute_raw(&delete_story, &[&id])
            .await
            .map_err(Error::from)
    }
}
