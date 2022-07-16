use std::convert::Infallible;

use sqlx::{query_as, Pool, Postgres};

use super::models;

pub async fn query_db(pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    let resp = match query_as::<_, models::Account>(
        "SELECT account.name, account.email, account.password FROM
        account JOIN feed ON feed.account_id = account.id;",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("no results recieved from db: {}", e);
            Vec::new()
        }
    };

    println!("returning {:?}", resp);

    return Ok(warp::reply::json(&resp));
}
