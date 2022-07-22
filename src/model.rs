use axum::http::StatusCode;
use sqlx::{PgPool, query_as, FromRow, postgres::PgRow, Row, query};
use async_graphql::{
    Context, Object, Schema, EmptySubscription,
};

pub type PaperSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Clone, FromRow)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

#[Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Clone)]
pub struct Feed {
    user_id: i32,
    url: String,
}

#[Object]
impl Feed {
    async fn user_id(&self) -> i32 {
        self.user_id
    }

    async fn url(&self) -> &str {
        &self.url
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn users (&self, ctx: &Context<'_>,) -> Vec<User> {
        let pool = ctx.data_unchecked::<PgPool>();
        let res = query(
            r#"
            select id, name, email from "user";
            "#)
            .map(|row: PgRow| {
                User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                }
            })
            .fetch_all(pool)
            .await
            .unwrap_or(Vec::new());
        res
    }

    async fn feeds ( &self, ctx: &Context<'_>, id: i32) -> Vec<Feed> {
        let pool = ctx.data_unchecked::<PgPool>();
        let res = query(
            r#"
            select url from feed
            where user_id = $1
            "#)
            .bind(id)
            .map(|row: PgRow| {
                Feed {
                    user_id: id,
                    url: row.try_get(0).unwrap_or(String::new())
                }
            })
            .fetch_all(pool)
            .await
            .unwrap_or(Vec::new());
        res
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(
        &self, ctx: &Context<'_>, name: String, email: String, password: String)
        -> i32 {
        let pool = ctx.data_unchecked::<PgPool>();
        let res = query(
                r#"
                insert into "user"
                (name, email, password)
                values ($1, $2, $3)
                returning id;
                "#
            )
            .bind(name)
            .bind(email)
            .bind(password)
            .fetch_one(pool)
            .await;
        match res {
            Ok(r) => r.get(0),
            Err(e) => {
                println!("{e}");
                -1
            }
        }
    }

    async fn create_feed(
        &self, ctx: &Context<'_>, id: i32, url: String)
        -> i32 {
        let pool = ctx.data_unchecked::<PgPool>();
        let res = query(
                r#"
                insert into feed
                (user_id, url)
                values ($1, $2)
                returning id;
                "#
            )
            .bind(id)
            .bind(url)
            .fetch_one(pool)
            .await;
        match res {
            Ok(r) => r.get(0),
            Err(e) => {
                println!("{e}");
                -1
            }
        }
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
