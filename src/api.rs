extern crate diesel;

use serde::Deserialize;
use upchallenge::{models::*, establish_connection};
use actix_web::{HttpRequest, HttpResponse, web, get};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;

// Lista os tweets do usuario
#[get("/user/{user}/tweets")]
pub fn user_tweets(params: web::Path<TweetsParams>) -> Result<HttpResponse, actix_web::Error> {
    use diesel::prelude::*;
    use upchallenge::schema::posts::dsl::*;
    use upchallenge::schema::users::dsl::*;

    let conn = establish_connection();
    let user = users.find(params.user).get_result::<User>(&conn).expect("User not found");
    let results = Post::belonging_to(&user).load::<Post>(&conn).expect("Error loading posts");

    HttpResponse::and_then(|mut response| match response.text() {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(error) => HttpResponse::InternalServerError()
    })
}

#[derive(Deserialize, Factory)]
pub struct TweetsParams {
    user: i64
}