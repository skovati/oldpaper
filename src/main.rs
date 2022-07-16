use std::env;
use std::error::Error;

use sqlx::postgres::PgPoolOptions;
use tokio::signal::unix;

mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_name = env::var("POSTGRES_DB")?;
    let db_ip = env::var("POSTGRES_IP")?;
    let user = env::var("POSTGRES_USER")?;
    let pw = env::var("POSTGRES_PASSWORD")?;
    let addr = format!("postgres://{}:{}@{}/{}", user, pw, db_ip, db_name);

    println!("connecting to postgres DB: {}", addr);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&addr)
        .await?;

    println!("connected to database!");

    println!("Warp server starting...");

    let routes = routes::get_api_routes(pool.clone());

    // wrap warp server in tokio signal processor, so we actually respond to SIGTERM
    let (_addr, fut) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            unix::signal(unix::SignalKind::terminate())
                .unwrap()
                .recv()
                .await
                .expect("failed to listen to shutdown signal");
        });

    fut.await;

    println!("Warp server shutting down");

    return Ok(());
}
