use std::env;
use std::error::Error;

use sqlx::Pool;
use sqlx::{postgres::PgPoolOptions, Postgres};
use tokio::signal::unix;
use warp::Filter;

mod handlers;
mod models;

fn with_db(db: Pool<Postgres>) -> impl Filter<Extract=(Pool<Postgres>, ), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_name = env::var("POSTGRES_DB")?;
    let db_ip = env::var("POSTGRES_IP")?;
    let user = env::var("POSTGRES_USER")?;
    let pw = env::var("POSTGRES_PASSWORD")?;
    let addr = format!("postgres://{}:{}@{}/{}", user, pw, db_ip, db_name);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&addr)
        .await?;

    let query = warp::get()
        .and(warp::path("query"))
        .and(with_db(pool.clone()))
        .and_then(handlers::query_db);

    println!("Warp server starting...");

    // wrap warp server in tokio signal processor, so we actually respond to SIGTERM
    let (_addr, fut) = warp::serve(query)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            unix::signal(unix::SignalKind::terminate()).unwrap().recv()
                .await
                .expect("failed to listen to shutdown signal");
        });

    fut.await;

    println!("Warp server shutting down");

    return Ok(());
}
