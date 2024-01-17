use async_graphql::{Context, EmptySubscription, ErrorExtensions, FieldResult, Object, Schema};

use crate::auth::UserInfo;
use crate::models::user::User;
use crate::services::lobby::LobbyService;
use crate::services::presence::PresenceService;
use crate::services::user::UserService;
use crate::services::Error;
use crate::{models::lobby::Lobby, DbPool};

pub struct Query;
pub struct Mutation;

fn db_pool<'a>(ctx: &Context<'a>) -> &'a DbPool {
    ctx.data::<DbPool>()
        .expect("No database connection pool in context")
}

fn presence_service<'a>(ctx: &Context<'a>) -> PresenceService<'a> {
    PresenceService::new(ctx.data::<redis::Client>().unwrap())
}

fn lobby_service<'a>(
    ctx: &Context<'a>,
    presence_service: &'a PresenceService<'a>,
) -> LobbyService<'a> {
    LobbyService::new(db_pool(ctx), presence_service)
}

#[Object]
impl Query {
    async fn profile(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let user_info = ctx.data::<UserInfo>().unwrap();

        let user = UserService::new(db_pool(ctx))
            .find(&user_info.user.id)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(user)
    }

    async fn lobby(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let presence_service = presence_service(ctx);
        let service = lobby_service(ctx, &presence_service);

        service
            .find(id, &ctx.data::<UserInfo>().unwrap().user)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))
    }
}

#[Object]
impl Mutation {
    async fn create_lobby(&self, ctx: &Context<'_>) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();

        let mut new_lobby = Lobby {
            host_id: user_info.user.id.clone(),
            ..Default::default()
        };

        new_lobby =
            lobby_service(ctx, &presence_service(ctx)).create(new_lobby, &user_info.user)?;

        Ok(new_lobby)
    }

    async fn configure_lobby(
        &self,
        ctx: &Context<'_>,
        id: String,
        guessing_time: i16,
    ) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let presence_service = presence_service(ctx);
        let service = lobby_service(ctx, &presence_service);
        let mut lobby = service.find(id, &user_info.user)?;
        lobby.guessing_time = guessing_time;
        lobby = service.configure(lobby, &user_info.user)?;

        Ok(lobby)
    }

    async fn join_lobby(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let presence_service = presence_service(ctx);
        let service = lobby_service(ctx, &presence_service);
        let lobby = service.join(id, &user_info.user)?;

        Ok(lobby)
    }

    async fn set_ready(&self, ctx: &Context<'_>, id: String, ready: bool) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let presence_service = presence_service(ctx);
        let service = lobby_service(ctx, &presence_service);
        let lobby = service.find(id, &user_info.user)?;
        let lobby = service.set_ready(lobby, &user_info.user, ready)?;

        Ok(lobby)
    }

    async fn set_content(&self, ctx: &Context<'_>, id: String, url: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let presence_service = presence_service(ctx);
        let service = lobby_service(ctx, &presence_service);
        let lobby = service.find(id, &user_info.user)?;
        let lobby = service.set_content(lobby, &user_info.user, url)?;

        Ok(lobby)
    }

    async fn start_game(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let presence_service = presence_service(ctx);
        let service = lobby_service(ctx, &presence_service);
        let mut lobby = service.find(id, &user_info.user)?;
        lobby = service.start_game(lobby, &user_info.user)?;

        Ok(lobby)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
