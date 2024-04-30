pub const FETCH: &str = "select id, name from stories where id = $1";
pub const SELECT: &str = "select id, name from stories order by id limit 10";
pub const INSERT: &str = "insert into stories (name) values ($1) returning id";
pub const DELETE: &str = "delete from stories where id = $1";
pub const UPDATE: &str = "update stories set name = $1 where id = $2";
