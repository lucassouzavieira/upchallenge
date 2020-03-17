extern crate diesel;

use serde::{Serialize, Deserialize};
use upchallenge::{models::*, establish_connection, create_post};
use actix_web::{HttpResponse, Responder, web, get, post};

// Lista os tweets do usuario
#[get("api/v1/user/{user}/tweets")]
pub async fn user_tweets(params: web::Path<TweetParams>) -> impl Responder {
    use diesel::prelude::*;
    use upchallenge::schema::users::dsl::*;

    let conn = establish_connection();
    let user = users.find(params.user).get_result::<User>(&conn).expect("User not found");
    let results = Post::belonging_to(&user).load::<Post>(&conn).expect("Error loading posts");

    if results.len() > 0 {
        return HttpResponse::Ok().json(results);
    }

    HttpResponse::InternalServerError().body("Some error happened. Check your .ENV configs")
}

// Exibe um tweet especifico
#[get("api/v1/user/{user}/tweets/{tweet}")]
pub async fn user_get_tweet(params: web::Path<GetTweetParams>) -> impl Responder {
    use diesel::prelude::*;
    use upchallenge::schema::posts::dsl::*;
    use upchallenge::schema::users::dsl::users;

    let conn = establish_connection();
    let user = users.find(params.user).get_result::<User>(&conn).expect("User not found");
    let results = Post::belonging_to(&user).filter(id.eq(params.tweet)).load::<Post>(&conn).expect("Error loading posts");

    return if results.len() <= 0 {
        HttpResponse::NotFound().json(Message { message: "Tweet not found".to_string() })
    } else {
        HttpResponse::Ok().json(results)
    };
}

// Cria um novo tweet
#[post("api/v1/user/{user}/tweets")]
pub async fn user_post_tweet(item: web::Json<CreatTweetForm>, params: web::Path<TweetParams>) -> impl Responder {
    let conn = establish_connection();
    let content = item.body.as_str().to_string();
    let created = create_post(&conn, params.user, content);
    HttpResponse::Created().json(created)
}

#[derive(Serialize, Deserialize)]
pub struct TweetParams {
    user: i64
}

#[derive(Serialize, Deserialize)]
pub struct GetTweetParams {
    user: i64,
    tweet: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    message: String
}

#[derive(Deserialize)]
pub struct CreatTweetForm {
    body: String,
}