pub const SELECT: &str =
    "select id, story_id, name, status from tasks where story_id = $1 order by id limit 10";
pub const FETCH: &str = "select id, story_id, name, status from tasks where id = $1";
pub const INSERT: &str = "insert into tasks (story_id, name) values ($1, $2) returning id, status";
pub const DELETE: &str = "delete from tasks where id = $1";
pub const DELETE_BY_STORY: &str = "delete from tasks where story_id = $1";
pub const UPDATE: &str = "update tasks set name = $1, status = $2 where id = $3 returning story_id";
