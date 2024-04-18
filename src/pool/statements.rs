use super::CustomPgConn;
use tokio_postgres::Statement;

// SQL queries for the stories table
const SQL_SELECT_STORY: &str = "select id, name from stories where id = $1";
const SQL_SELECT_STORIES: &str = "select id, name from stories order by id limit 10";
const SQL_INSERT_STORY: &str = "insert into stories (name) values ($1) returning id";
const SQL_DELETE_STORY: &str = "delete from stories where id = $1";

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Name {
    SelectStories,
    SelectStory,
    InsertStory,
    DeleteStory,
}

/// Grouped prepared statements
pub struct Statements {
    pub select_story: Statement,
    pub select_stories: Statement,
    pub insert_story: Statement,
    pub delete_story: Statement,
}

impl Statements {
    pub async fn prepare(conn: &CustomPgConn) -> Self {
        Self {
            select_story: conn.prepare(SQL_SELECT_STORY).await.unwrap(),
            select_stories: conn.prepare(SQL_SELECT_STORIES).await.unwrap(),
            insert_story: conn.prepare(SQL_INSERT_STORY).await.unwrap(),
            delete_story: conn.prepare(SQL_DELETE_STORY).await.unwrap(),
        }
    }
}
