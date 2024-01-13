use crate::{
    db_schema::{lobbies::dsl::*, lobbies_players},
    models::{lobby::LobbyPlayers, user::User},
};
use diesel::{associations::HasTable, prelude::*};

use crate::{models::lobby::Lobby, DbPool};

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

        // TODO: run this in a transaction

        diesel::insert_into(lobbies)
            .values(&lobby)
            .execute(&mut conn)
            .map_err(Error::Db)?;

        self.join(lobby.id.clone(), user)?;

        Ok(lobby)
    }

    pub fn find(&self, lobby_id: String, user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        let lobby = lobbies
            .filter(id.eq(lobby_id))
            .limit(1)
            .get_result::<Lobby>(&mut conn)
            .map_err(Error::Db)?;

        // TODO: seperate it into it's own heartbeat mechanism
        self.presence_service.heartbeat(&lobby, user)?;

        self.clear_inactive_players(&lobby)?;

        Ok(lobby)
    }

    pub fn join(&self, lobby_id: String, user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        let lobby = self.find(lobby_id, user)?;

        let lobby_player = LobbyPlayers {
            lobby_id: lobby.id.clone(),
            player_id: user.id.clone(),
            is_ready: false,
            contents_id: None,
            created_at: chrono::Utc::now(),
        };

        diesel::insert_into(lobbies_players::table)
            .values(lobby_player)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .map_err(Error::Db)?;

        self.presence_service.heartbeat(&lobby, user)?;

        Ok(lobby)
    }

    pub fn configure(&self, lobby: Lobby, _user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        // TODO: verifiy user has the permission to configure the lobby!

        diesel::update(lobbies)
            .filter(id.eq(lobby.id.clone()))
            .set(&lobby)
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(lobby)
    }

    pub fn start_game(&self, lobby: Lobby, _user: &User) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        // TODO: verifiy user has the permission to start the game!

        diesel::update(lobbies)
            .filter(id.eq(lobby.id.clone()))
            .set(started_at.eq(chrono::Utc::now().naive_utc()))
            .execute(&mut conn)
            .map_err(Error::Db)?;

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

        // TODO: delete lobby (and notify users) if host is no longer present

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
