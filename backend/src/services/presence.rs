use redis::Commands;

use crate::models::{lobby::Lobby, user::User};

use super::Error;

#[derive(Clone)]
pub struct PresenceService {
    pub(crate) redis: redis::Client,
}

impl PresenceService {
    pub fn new(redis: redis::Client) -> Self {
        Self { redis }
    }

    pub fn heartbeat(&self, lobby: &Lobby, user: &User) -> Result<(), Error> {
        let mut redis = self
            .redis
            .get_connection()
            .map_err(Error::RedisConnection)?;

        redis
            .set_ex(format!("lobby:{}|player-id:{}", lobby.id, user.id), 42, 5)
            .map_err(Error::RedisConnection)?;

        Ok(())
    }

    pub fn present_user_ids(&self, lobby: &Lobby) -> Result<Vec<String>, Error> {
        let mut redis = self
            .redis
            .get_connection()
            .map_err(Error::RedisConnection)?;

        let keys: Vec<String> = redis
            .keys(format!("lobby:{}|player-id:*", lobby.id))
            .map_err(Error::RedisConnection)?;

        let mut users = Vec::new();

        for key in keys {
            let user_id = key.split(':').last().unwrap().to_owned();

            users.push(user_id);
        }

        Ok(users)
    }
}
