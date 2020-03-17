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

pub fn create_post(conn: &MysqlConnection, user_id: i64, body: String) -> Post {
    use self::schema::posts::dsl::{id, posts};

    let post = NewPost { user_id, body: body.to_string() };

    diesel::insert_into(posts)
        .values(&post)
        .execute(conn)
        .expect("Error saving new post");

    posts.order(id.desc()).first(conn).unwrap()
}
