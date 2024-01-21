use crate::{
    db_schema::contents::dsl::*,
    models::{content::Contents, lobby::Lobby, user::User},
    DbPool,
};

use diesel::prelude::*;

use super::Error;

pub struct ContentService {
    db_pool: DbPool,
}

impl ContentService {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub fn find(&self, lobby: &Lobby, user: &User) -> Result<Option<Contents>, Error> {
        let mut conn = self.db_pool.get()?;

        let content = contents
            .filter(lobby_id.eq(lobby.id.clone()))
            .filter(user_id.eq(user.id.clone()))
            .first::<Contents>(&mut conn)
            .optional()
            .map_err(Error::Db)?;

        Ok(content)
    }

    pub fn current(&self, lobby: &Lobby) -> Result<Option<Contents>, Error> {
        let mut conn = self.db_pool.get()?;

        if lobby.current_user_id.is_none() {
            return Ok(None);
        }

        let content = contents
            .filter(lobby_id.eq(lobby.id.clone()))
            .filter(user_id.eq(lobby.current_user_id.clone().unwrap()))
            .first::<Contents>(&mut conn)
            .optional()
            .map_err(Error::Db)?;

        Ok(content)
    }
}
