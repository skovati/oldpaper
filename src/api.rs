use sqlx::{Pool, Postgres};
use warp::Filter;

use crate::handlers;

pub fn with_pool(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn get_api_routes(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /feeds
    let feed = warp::get()
        .and(warp::path("query"))
        .and(with_pool(pool.clone()))
        .and_then(handlers::query_db);

    // POST /subscribe
    // json body { url: string }

    // POST /unsubscribe/URL
    // json body { url: string }

    // POST /delete

    let routes = feed;
    return routes;
}
