use std::env;
use std::error::Error;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{query, Row};
use warp::Filter;

struct Account {
    name: String,
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_name = env::var("POSTGRES_DB")?;
    let db_ip = env::var("POSTGRES_IP")?;
    let user = env::var("POSTGRES_USER")?;
    let pw = env::var("POSTGRES_PASSWORD")?;
    let addr = format!("postgres://{}:{}@{}/{}", user, pw, db_ip, db_name);
    println!("{}", addr);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&addr)
        .await?;

    let _rows = query(
        "INSERT INTO account
        (name, email, password)
        VALUES ('Luke', 'mail@skovati.dev', 'secure_hash');",
    )
    .execute(&pool)
    .await?;

    let tmp = query("SELECT * from account;").fetch_all(&pool).await?;

    let res = tmp.get(0).unwrap();

    println!("{}", res.get::<String, _>(0));

    return Ok(());
}
