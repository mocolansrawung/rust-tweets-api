use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;

use responses::Tweet;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_tweet(
    pool: &Pool<Sqlite>,
    name: &String,
    description: &String,
) -> DBResult<i64> {
    let mut connection = pool
        .acquire()
        .await?;
    let id = sqlx::query_as!(
            Tweet,
            r#"
        INSERT INTO tweets (name, description) VALUES (?, ?);
        "#,
            name,
            description
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();
        Ok(id)
}

pub async fn get_tweet(pool: &Pool<Sqlite>, id: i64) -> DBResult<Tweet> {
    let mut connection = pool.acquire()
        .await?;
    let tweet = sqlx::query_as!(
        Tweet,
        r#"
        SELECT id, name, description from tweets
        WHERE id = ?;
        "#,
            id
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(tweet)
}

pub async fn get_tweets(pool: &Pool<Sqlite>) -> DBResult<Vec<Tweet>> {
    let mut connection = pool.acquire()
        .await
        .unwrap();
    let tweets = sqlx::query_as::<_, Tweet>(
        r#"
        select id, name, description from tweets;
        "#
    )
        .fetch_all(&mut connection)
        .await?;
        Ok(tweets)
}