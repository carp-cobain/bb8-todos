pub const FETCH: &str = "select id, story_id, name, status from tasks where id = $1";
pub const DELETE: &str = "delete from tasks where id = $1";
pub const DELETE_BY_STORY: &str = "delete from tasks where story_id = $1";
pub const UPDATE: &str = "update tasks set name = $1, status = $2 where id = $3 returning story_id";
pub const INSERT: &str =
    "insert into tasks (story_id, name, status) values ($1, $2, $3) returning id";
pub const SELECT: &str = r#"
select id, story_id, name, status from tasks where story_id = $1 and id >= $2 order by id limit 10
"#;
