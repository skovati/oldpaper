use std::convert::Infallible;

use sqlx::{Pool, Postgres, query, Row};
use warp::http::StatusCode;

use super::models;

pub async fn get_feeds(id: i32, pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    let resp = match query(
        r#"
        SELECT account.name, feed.url FROM
        account JOIN feed ON feed.account_id = account.id
        WHERE account.id = $1;
        "#
    )
    .bind(id)
    .fetch_all(&pool)
    .await {
        Ok(r) => {
            println!("query from db");
            r
        },
        Err(e) => {
            println!("no results recieved from db: {}", e);
            Vec::new()
        }
    };

    let mut json = models::FeedResponse {
        name: String::new(),
        feeds: Vec::new()
    };

    resp.iter().for_each(|f| {
        json.name = f.try_get("name").unwrap();
        json.feeds.push(f.try_get("url").unwrap());
    });

    return Ok(warp::reply::json(&json));
}

pub async fn add_feed(sub: models::Subscription, pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    match query(
        r#"
        INSERT INTO feed (account_id, url)
        VALUES ($1, $2)
        RETURNING id;
        "#
    )
    .bind(sub.id as i32)
    .bind(sub.url)
    .fetch_one(&pool)
    .await {
        Ok(r) => {
            println!("{} id feed added", r.try_get::<i32, _>("id").unwrap());
        },
        Err(e) => {
            println!("error from db: {}", e);
        }
    };

    return Ok(StatusCode::CREATED);
}
