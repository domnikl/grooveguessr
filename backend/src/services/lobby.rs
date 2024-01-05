use crate::db_schema::lobbies::dsl::*;
use diesel::prelude::*;

use crate::{schemas::lobby::Lobby, DbPool};

use super::Error;

pub struct LobbyService<'a> {
    db_pool: &'a DbPool,
}

impl<'a> LobbyService<'a> {
    pub fn new(db_pool: &'a DbPool) -> Self {
        Self { db_pool }
    }

    pub fn create(&self, lobby: Lobby) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get().map_err(Error::DbConnectionError)?;

        diesel::insert_into(lobbies)
            .values(&lobby)
            .execute(&mut conn)
            .map_err(Error::DbError)?;

        Ok(lobby)
    }

    pub fn find(&self, lobby_id: String) -> Result<Lobby, Error> {
        let mut conn = self.db_pool.get()?;

        let lobby = lobbies
            .filter(id.eq(lobby_id))
            .limit(1)
            .get_result::<Lobby>(&mut conn)
            .map_err(Error::DbError)?;

        Ok(lobby)
    }
}
