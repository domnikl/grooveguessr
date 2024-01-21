use async_graphql::{Context, EmptySubscription, ErrorExtensions, FieldResult, Object, Schema};

use crate::auth::UserInfo;
use crate::models::lobby::Lobby;
use crate::models::user::User;
use crate::services::lobby::LobbyService;
use crate::services::user::UserService;
use crate::services::Error;

pub struct Query;
pub struct Mutation;

#[Object]
impl Query {
    async fn profile(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let user_service = ctx.data::<UserService>().unwrap();

        let user = user_service
            .find(&user_info.user.id)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(user)
    }

    async fn lobby(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let service = ctx.data::<LobbyService>().unwrap();

        service
            .find(id, &ctx.data::<UserInfo>().unwrap().user)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))
    }
}

#[Object]
impl Mutation {
    async fn create_lobby(&self, ctx: &Context<'_>) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<LobbyService>().unwrap();

        let mut new_lobby = Lobby {
            host_id: user_info.user.id.clone(),
            ..Default::default()
        };

        new_lobby = service.create(new_lobby, &user_info.user)?;

        Ok(new_lobby)
    }

    async fn configure_lobby(
        &self,
        ctx: &Context<'_>,
        id: String,
        guessing_time: i16,
    ) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<LobbyService>().unwrap();

        let mut lobby = service.find(id, &user_info.user)?;
        lobby.guessing_time = guessing_time;
        lobby = service.configure(lobby, &user_info.user)?;

        Ok(lobby)
    }

    async fn join_lobby(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<LobbyService>().unwrap();
        let lobby = service.find(id, &user_info.user)?;
        let lobby = service.join(&lobby, &user_info.user)?;

        Ok(lobby)
    }

    async fn set_ready(&self, ctx: &Context<'_>, id: String, ready: bool) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<LobbyService>().unwrap();
        let lobby = service.find(id, &user_info.user)?;
        let lobby = service.set_ready(lobby, &user_info.user, ready)?;

        Ok(lobby)
    }

    async fn set_content(&self, ctx: &Context<'_>, id: String, url: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<LobbyService>().unwrap();
        let lobby = service.find(id, &user_info.user)?;
        let lobby = service.set_content(lobby, &user_info.user, url)?;

        Ok(lobby)
    }

    async fn start_game(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<LobbyService>().unwrap();
        let mut lobby = service.find(id, &user_info.user)?;
        lobby = service.start_game(lobby, &user_info.user)?;

        Ok(lobby)
    }

    async fn set_name(&self, ctx: &Context<'_>, name: String) -> FieldResult<User> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let service = ctx.data::<UserService>().unwrap();
        let mut user = service.find(&user_info.user.id)?;
        user.name = name;

        user = service.save(user)?;

        Ok(user)
    }

    async fn guess(
        &self,
        ctx: &Context<'_>,
        id: String,
        round_index: usize,
        guessed_user_id: String,
    ) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();
        let user_service = ctx.data::<UserService>().unwrap();
        let lobby_service = ctx.data::<LobbyService>().unwrap();
        let guessed_user = user_service.find(&guessed_user_id)?;
        let lobby = lobby_service.find(id, &user_info.user)?;

        lobby_service.guess(&lobby, round_index, &user_info.user, &guessed_user)?;

        Ok(lobby)
    }

    async fn forward(&self, ctx: &Context<'_>, id: String) -> FieldResult<Lobby> {
        let user_info = ctx.data::<UserInfo>().unwrap();

        let lobby_service = ctx.data::<LobbyService>().unwrap();
        let lobby = lobby_service.find(id, &user_info.user)?;
        let lobby = lobby_service.forward(lobby, &user_info.user)?;

        Ok(lobby)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
