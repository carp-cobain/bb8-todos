pub const FETCH: &str = "select id, name from stories where id = $1";
pub const INSERT: &str = "insert into stories (name) values ($1) returning id";
pub const DELETE: &str = "delete from stories where id = $1";
pub const UPDATE: &str = "update stories set name = $1 where id = $2";

pub const SELECT: &str = r#"
with cursor as (
    select id from stories
    where id = $1
    order by id limit 1
), previous_page as (
    select id, name from stories
    where id < (select id from cursor)
    order by id desc limit 100
), current_next_page as (
    select id, name from stories
    where id >= (select id from cursor)
    order by id limit 101
) (
    select id, name, 'prev' as label from previous_page
    order by id limit 1
) union all (
    select id, name, 'current' as label from current_next_page
    order by id limit 100
) union all (
    select id, name, 'next' as label from current_next_page
    order by id limit 1 offset 100
)"#;
