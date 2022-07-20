use async_graphql::{
    EmptySubscription, Schema,
};
use axum::{
    extract::Extension,
    routing::get, Router,
};

mod model;
mod handler;
use crate::model::{QueryRoot, MutationRoot, Storage};


#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
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
