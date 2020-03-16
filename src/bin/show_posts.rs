extern crate upchallenge;
extern crate diesel;

use self::models::*;
use diesel::prelude::*;
use upchallenge::*;

fn main() {
    use upchallenge::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}