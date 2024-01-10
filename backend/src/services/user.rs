use crate::db_schema::users;
use diesel::prelude::*;

use crate::{models::user::User, DbPool};

use super::Error;

pub struct UserService<'a> {
    db_pool: &'a DbPool,
}

impl<'a> UserService<'a> {
    pub fn new(db_pool: &'a DbPool) -> Self {
        Self { db_pool }
    }

    pub fn register(&self, user: User) -> Result<User, Error> {
        let mut conn = self.db_pool.get().map_err(Error::DbConnection)?;

        diesel::insert_into(users::table)
            .values(&user)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(user)
    }

    pub fn find(&self, user_id: &str) -> Result<User, Error> {
        let mut conn = self.db_pool.get()?;

        let user = users::table
            .filter(users::id.eq(user_id))
            .limit(1)
            .get_result::<User>(&mut conn)
            .map_err(Error::Db)?;

        Ok(user)
    }
}
