use async_graphql::{Context, EmptySubscription, ErrorExtensions, FieldResult, Object, Schema};

use crate::auth::UserInfo;
use crate::models::user::User;
use crate::services::lobby::LobbyService;
use crate::services::user::UserService;
use crate::services::Error;
use crate::{models::lobby::Lobby, DbPool};

pub struct Query;
pub struct Mutation;

fn db_pool<'a>(ctx: &Context<'a>) -> &'a DbPool {
    ctx.data::<DbPool>()
        .expect("No database connection pool in context")
}

#[Object]
impl Query {
    async fn profile(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let user_info = ctx.data::<UserInfo>().unwrap();

        let user = UserService::new(db_pool(ctx))
            .find(user_info.user.id)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(user)
    }

    async fn lobby(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        LobbyService::new(db_pool(ctx))
            .find(id)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))
    }
}

#[Object]
impl Mutation {
    async fn create_lobby(&self, ctx: &Context<'_>) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();

        let mut new_lobby = Lobby {
            host_id: user_info.user.id,
            ..Default::default()
        };

        new_lobby = LobbyService::new(db_pool(ctx)).create(new_lobby)?;

        Ok(new_lobby)
    }

    async fn configure_lobby(
        &self,
        ctx: &Context<'_>,
        id: String,
        guessing_time: i16,
    ) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = LobbyService::new(db_pool(ctx));
        let mut lobby = service.find(id)?;
        lobby.guessing_time = guessing_time;
        lobby = service.configure(lobby, &user_info.user)?;

        Ok(lobby)
    }

    async fn start_game(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = LobbyService::new(db_pool(ctx));
        let mut lobby = service.find(id)?;
        lobby = service.start_game(lobby, &user_info.user)?;

        Ok(lobby)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
