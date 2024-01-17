use crate::{
    db_schema::contents::dsl::*,
    models::{content::Contents, lobby::Lobby, user::User},
    DbPool,
};

use diesel::prelude::*;

use super::Error;

pub struct ContentService<'a> {
    db_pool: &'a DbPool,
}

impl<'a> ContentService<'a> {
    pub fn new(db_pool: &'a DbPool) -> Self {
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
}
