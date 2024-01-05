use async_graphql::{Context, EmptySubscription, ErrorExtensions, FieldResult, Object, Schema};

use crate::services::lobby::LobbyService;
use crate::services::Error;
use crate::{schemas::lobby::Lobby, DbPool};

pub struct Query;
pub struct Mutation;

fn db_pool<'a>(ctx: &Context<'a>) -> &'a DbPool {
    ctx.data_unchecked::<DbPool>()
}

#[derive(async_graphql::InputObject)]
struct LobbyInput {
    id: String,
}

#[Object]
impl Query {
    async fn lobby(&self, ctx: &Context<'_>, lobby_input: LobbyInput) -> FieldResult<Lobby> {
        LobbyService::new(db_pool(ctx))
            .find(lobby_input.id)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))
    }
}

#[Object]
impl Mutation {
    async fn create_lobby(&self, ctx: &Context<'_>) -> FieldResult<Lobby> {
        let mut new_lobby = Lobby::default();
        new_lobby = LobbyService::new(db_pool(ctx)).create(new_lobby)?;

        Ok(new_lobby)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
