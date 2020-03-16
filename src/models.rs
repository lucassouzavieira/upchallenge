#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}