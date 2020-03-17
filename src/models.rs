use serde::{Serialize, Deserialize};

#[derive(Debug, Identifiable, Queryable, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: i64,
    name: String,
    email: String,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize, PartialEq)]
#[belongs_to(User, foreign_key = "user_id")]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub body: String,
}

use super::schema::{posts, users};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Insertable, Associations)]
#[table_name = "posts"]
pub struct NewPost {
    pub user_id: i64,
    pub body: String,
}