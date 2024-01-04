use async_graphql::{Context, EmptyMutation, EmptySubscription, FieldResult, Object, Schema};

pub struct Query;

#[Object]
impl Query {
    async fn videos(&self, _ctx: &Context<'_>) -> FieldResult<String> {
        Ok("Hello World".to_string())
    }
}

pub type ProjectSchema = Schema<Query, EmptyMutation, EmptySubscription>;
