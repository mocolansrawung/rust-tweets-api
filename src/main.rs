#[macro_use]
extern crate rocket;
mod database;

use database::requests::TweetRequest;
use database::responses::Tweet;
use database::{create_tweet, get_tweet, get_tweets, DBResult};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};

#[post("/tweets", format = "json", data = "<tweet>")]
async fn create(tweet: Json<TweetRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Tweet>> {
    let id = create_tweet(pool, &tweet.name, &tweet.description).await?;
    let tweet = get_tweet(pool, id).await?;
    Ok(Json(tweet))
}

#[get("/tweets")]
async fn index(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<Tweet>>> {
    let tweets = get_tweets(pool).await?;
    Ok(Json(tweets))
}

#[get("/tweets/<id>")]
async fn detail(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Tweet>> {
    let tweet = get_tweet(pool, id).await?;
    Ok(Json(tweet))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut config = sqlx::sqlite::SqliteConnectOptions::new();
    config = config.filename("data.db");
    let pool = SqlitePool::connect_with(config)
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let _rocket = rocket::build()
        .mount("/", routes![index, create, detail])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}