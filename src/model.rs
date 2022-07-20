use std::sync::Arc;

use async_graphql::{
    Context, Object, Schema, EmptySubscription,
};
use slab::Slab;
use tokio::sync::Mutex;

pub type PaperSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone)]
pub struct User {
    id: usize,
    name: String,
    email: String,
}

#[Object]
impl User {
    async fn id(&self) -> usize {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }
}

pub struct QueryRoot;

pub type Storage = Arc<Mutex<Slab<User>>>;

#[Object]
impl QueryRoot {
    async fn users (&self, ctx: &Context<'_>,) -> Vec<User> {
        let users = ctx.data_unchecked::<Storage>().lock().await;
        users.iter().map(|(_, book)| book).cloned().collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(
        &self, ctx: &Context<'_>, name: String, email: String)
        -> usize {
        let mut users = ctx.data_unchecked::<Storage>().lock().await;
        let entry = users.vacant_entry();
        let id = entry.key().into();
        let user = User {
            id,
            name,
            email
        };
        entry.insert(user);
        id
    }
}
