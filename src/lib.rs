#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&db_url).
        expect(&format!("Error connecting to {}", db_url))
}

use models::{Post, NewPost};

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
    use self::schema::posts::dsl::{id, posts};

    let post = NewPost { title, body };

    diesel::insert_into(posts)
        .values(&post)
        .execute(conn)
        .expect("Error savind new post");

    posts.order(id.desc()).first(conn).unwrap()
}
