use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Enum, FieldResult, Interface, Object,
};
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn test_query(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "id of the human")] id: String,
        ) -> Option<String> {
            Some("test data".to_string())
        }
}
