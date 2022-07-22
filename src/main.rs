use async_graphql::{
    EmptySubscription, Schema,
};
use axum::{
    extract::Extension,
    routing::get, Router,
};
use sqlx::postgres::PgPoolOptions;

mod model;
mod handler;
use crate::model::{Query, Mutation};

#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or("oldpaper://oldpaper:oldpaper@127.0.0.1:15432".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    let app = Router::new()
        .route("/gql",
            get(handler::graphql_playground)
            .post(handler::graphql_handler))
        .route("/health", get(handler::health))
        .route("/", get(handler::index))
        .layer(Extension(schema));

    println!("serving playground @ http://localhost:8080/gql");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
