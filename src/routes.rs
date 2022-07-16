use sqlx::{Pool, Postgres};
use warp::Filter;

use crate::handlers;
use crate::models;

fn with_pool(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn sub_json_body() -> impl Filter<Extract = (models::Subscription,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn id_json_body() -> impl Filter<Extract = (i32, ), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_api_routes(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /feeds
    let feeds = warp::get()
        .and(warp::path("query"))
        .and(id_json_body())
        .and(with_pool(pool.clone()))
        .and_then(handlers::get_feeds);

    // POST /subscribe
    // json body { url: string }
    let subscribe = warp::post()
        .and(warp::path("subscribe"))
        .and(sub_json_body())
        .and(with_pool(pool.clone()))
        .and_then(handlers::add_feed);

    // POST /unsubscribe/URL
    // json body { url: string }

    // POST /delete

    let routes = feeds
        .or(subscribe);
    return routes;
}
