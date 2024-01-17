use crate::{
    db_schema::{
        contents::{self},
        lobbies::dsl::*,
        lobbies_players,
    },
    models::{content::Contents, lobby::LobbyPlayers, user::User},
};
use crate::{models::lobby::Lobby, DbPool};
use diesel::{associations::HasTable, prelude::*, upsert::on_constraint};
use rand::seq::SliceRandom;

use super::{presence::PresenceService, Error};

pub struct LobbyService<'a> {
    db_pool: &'a DbPool,
    presence_service: &'a PresenceService<'a>,
}

impl<'a> LobbyService<'a> {
    pub fn new(db_pool: &'a DbPool, presence_service: &'a PresenceService) -> Self {
        Self {
            db_pool,
            presence_service,
        }
    }

    pub fn create(&self, lobby: Lobby, user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get().map_err(Error::DbConnection)?;

        diesel::insert_into(lobbies)
            .values(&lobby)
            .execute(&mut conn)
            .map_err(Error::Db)?;

        self.join(&lobby, user)?;

        Ok(lobby)
    }

    pub fn find(&self, by_lobby_id: String, user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        let lobby = lobbies
            .filter(id.eq(by_lobby_id))
            .limit(1)
            .get_result::<Lobby>(&mut conn)
            .map_err(Error::Db)?;

        // TODO: seperate it into it's own heartbeat mechanism
        self.presence_service.heartbeat(&lobby, user)?;

        self.clear_inactive_players(&lobby)?;

        Ok(lobby)
    }

    pub fn join(&self, lobby: &Lobby, user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        let lobby_player = LobbyPlayers {
            lobby_id: lobby.id.clone(),
            player_id: user.id.clone(),
            is_ready: false,
            created_at: chrono::Utc::now(),
        };

        diesel::insert_into(lobbies_players::table)
            .values(lobby_player)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .map_err(Error::Db)?;

        self.presence_service.heartbeat(lobby, user)?;

        Ok(lobby.clone())
    }

    pub fn configure(&self, lobby: Lobby, _user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        if lobby.host_id != _user.id {
            return Err(Error::Unauthorized);
        }

        diesel::update(lobbies)
            .filter(id.eq(lobby.id.clone()))
            .set(&lobby)
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(lobby)
    }

    pub fn start_game(&self, lobby: Lobby, user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        if lobby.host_id != user.id {
            return Err(Error::Unauthorized);
        }

        if lobby.started_at.is_some() {
            return Err(Error::GameAlreadyStarted);
        }

        // TODO: run it in a transaction
        let players = lobbies_players::table
            .filter(lobbies_players::lobby_id.eq(&lobby.id))
            .get_results::<LobbyPlayers>(&mut conn)
            .map_err(Error::Db)?;

        if players.len() < 3 {
            return Err(Error::NotEnoughPlayers);
        }

        let player_contents = contents::table
            .filter(contents::lobby_id.eq(&lobby.id))
            .get_results::<Contents>(&mut conn)
            .map_err(Error::Db)?;

        if players.len() != player_contents.len() {
            return Err(Error::NotEveryoneHasContent);
        }

        // shuffle the player ids to determine the order of guesses
        let mut shuffled_ids = player_contents
            .into_iter()
            .map(|c| c.user_id)
            .collect::<Vec<String>>();

        shuffled_ids.shuffle(&mut rand::thread_rng());

        diesel::update(lobbies)
            .filter(id.eq(lobby.id.clone()))
            .set((
                started_at.eq(chrono::Utc::now().naive_utc()),
                sequence.eq(shuffled_ids.join(",")),
                current_user_id.eq(shuffled_ids[0].clone()),
            ))
            .execute(&mut conn)
            .map_err(Error::Db)?;

        // TODO: retrieve current content from lobby
        // TODO: mutation to forward to the next content
        // TODO: mutation to guess the current content

        Ok(lobby)
    }

    pub fn set_ready(&self, lobby: Lobby, user: &User, ready: bool) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        diesel::update(lobbies_players::table)
            .filter(
                lobbies_players::lobby_id
                    .eq(&lobby.id)
                    .and(lobbies_players::player_id.eq(&user.id)),
            )
            .set(lobbies_players::is_ready.eq(&ready))
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(lobby)
    }

    pub fn set_content(&self, lobby: Lobby, user: &User, url: String) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        diesel::insert_into(contents::table)
            .values((
                contents::user_id.eq(user.id.clone()),
                contents::data.eq(url.clone()),
                contents::type_.eq("url"),
                contents::lobby_id.eq(lobby.id.clone()),
                contents::created_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .on_conflict(on_constraint("contents_pkey"))
            .do_update()
            .set((contents::data.eq(url), contents::type_.eq("url")))
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(lobby)
    }

    pub fn clear_inactive_players(&self, lobby: &Lobby) -> Result<(), Error> {
        let mut conn = self.db_pool.get()?;

        let present_user_ids = self.presence_service.present_user_ids(lobby)?;

        diesel::delete(
            lobbies_players::table.filter(
                lobbies_players::lobby_id
                    .eq(&lobby.id)
                    .and(lobbies_players::player_id.ne_all(&present_user_ids)),
            ),
        )
        .execute(&mut conn)
        .map_err(Error::Db)?;

        // TODO: handle closing of a lobby gracefully in the client

        if !present_user_ids.contains(&lobby.host_id) {
            diesel::delete(
                lobbies::table().filter(id.eq(&lobby.id).and(host_id.eq(&lobby.host_id))),
            )
            .execute(&mut conn)
            .map_err(Error::Db)?;
        }

        Ok(())
    }
}
