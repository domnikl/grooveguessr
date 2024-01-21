use std::clone;

use async_graphql::*;
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    auth::UserInfo,
    db_schema::lobbies::{self, current_user_id},
    db_schema::lobbies_players,
    services::{
        content::ContentService, lobby::LobbyService, presence::PresenceService, user::UserService,
        Error,
    },
    DbPool,
};

use super::{
    content::Contents,
    user::{self, User},
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    SimpleObject,
    Queryable,
    Identifiable,
    Selectable,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = lobbies)]
#[graphql(complex)]
pub struct Lobby {
    pub id: String,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub guessing_time: i16,
    pub host_id: String,
    #[graphql(skip)]
    pub sequence: Option<String>,
    pub current_user_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

impl Default for Lobby {
    fn default() -> Self {
        Self {
            id: generate_random_string(10),
            started_at: None,
            guessing_time: 80,
            sequence: None,
            current_user_id: None,
            created_at: chrono::Utc::now().naive_utc(),
            host_id: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
struct Player {
    id: String,
    name: String,
    is_ready: bool,
}

impl Lobby {
    fn current_user_index(&self) -> Result<usize, Error> {
        match self.current_user_id.clone() {
            None => Err(Error::GameNotStarted),
            Some(user_id) => match self.sequence.clone() {
                Some(s) => Ok(s
                    .split(',')
                    .position(|s| s == user_id)
                    .ok_or(Error::GameNotStarted)?),
                None => Err(Error::GameNotStarted),
            },
        }
    }

    pub fn forward(mut self) -> Result<Self, Error> {
        let current_user_index = self.current_user_index()?;

        let sequence = match self.sequence {
            Some(ref sequence) => Ok(sequence.split(',').collect::<Vec<&str>>()),
            None => Err(Error::GameNotStarted),
        }?;

        if current_user_index == sequence.len() - 1 {
            return Err(Error::GameAlreadyFinished);
        }

        self.current_user_id = Some(sequence[current_user_index + 1].to_string());

        Ok(self)
    }
}

#[ComplexObject]
impl Lobby {
    async fn host(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let db_pool = ctx
            .data::<DbPool>()
            .expect("No database connection pool in context");

        let user = UserService::new(db_pool)
            .find(&self.host_id)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(user)
    }

    async fn content(&self, ctx: &Context<'_>) -> FieldResult<Option<Contents>> {
        let db_pool = ctx
            .data::<DbPool>()
            .expect("No database connection pool in context");

        let content = ContentService::new(db_pool)
            .find(self, &ctx.data::<UserInfo>().unwrap().user)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(content)
    }

    async fn current_content(&self, ctx: &Context<'_>) -> FieldResult<Option<Contents>> {
        let db_pool = ctx
            .data::<DbPool>()
            .expect("No database connection pool in context");

        let content = ContentService::new(db_pool)
            .current(self)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(content)
    }

    async fn guesses(&self, ctx: &Context<'_>) -> FieldResult<Vec<String>> {
        let db_pool = ctx
            .data::<DbPool>()
            .expect("No database connection pool in context");

        let redis = ctx
            .data::<redis::Client>()
            .expect("No redis connection pool in context");

        let user = ctx.data::<UserInfo>().unwrap().user.clone();

        let presence_service = PresenceService::new(redis);

        let guesses = LobbyService::new(db_pool, &presence_service)
            .guesses(self, &user)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        Ok(guesses)
    }

    async fn round_index(&self, ctx: &Context<'_>) -> FieldResult<Option<usize>> {
        let db_pool = ctx
            .data::<DbPool>()
            .expect("No database connection pool in context");

        let sequence = match self.sequence {
            Some(ref sequence) => sequence.split(',').collect::<Vec<&str>>(),
            None => return Ok(None),
        };

        let content = ContentService::new(db_pool)
            .current(self)
            .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

        match content {
            Some(content) => Ok(sequence.iter().position(|&s| s == content.user_id)),
            None => Ok(None),
        }
    }

    async fn players(&self, ctx: &Context<'_>) -> FieldResult<Vec<Player>> {
        let db_pool = ctx
            .data::<DbPool>()
            .expect("No database connection pool in context");

        let mut conn = db_pool.get()?;

        let players = lobbies_players::table
            .filter(lobbies_players::lobby_id.eq(&self.id))
            .order(lobbies_players::created_at.asc())
            .get_results::<LobbyPlayers>(&mut conn)
            .map_err(Error::Db)?;

        let mut users: Vec<Player> = Vec::new();

        for player in players {
            let user = UserService::new(db_pool)
                .find(&player.player_id)
                .map_err(|err: Error| err.extend_with(|_, e| e.set("code", 404)))?;

            let new_player = Player {
                id: user.id.clone(),
                name: user.name.clone(),
                is_ready: player.is_ready,
            };

            users.push(new_player);
        }

        Ok(users)
    }
}

#[derive(Identifiable, Selectable, Queryable, Associations, Insertable, Debug)]
#[diesel(belongs_to(Lobby))]
#[diesel(belongs_to(User, foreign_key = player_id))]
#[diesel(table_name = lobbies_players)]
#[diesel(primary_key(lobby_id, player_id))]
pub struct LobbyPlayers {
    pub lobby_id: String,
    pub player_id: String,
    pub is_ready: bool,
    pub guesses: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use crate::models::lobby;

    #[test]
    fn generate_random_string() {
        let random_string = lobby::generate_random_string(10);

        assert_eq!(random_string.len(), 10);
        assert!(random_string.chars().all(char::is_alphanumeric));
        assert!(random_string.chars().any(char::is_numeric));
    }

    #[test]
    fn lobby_has_default() {
        let default_lobby = lobby::Lobby::default();

        assert_eq!(default_lobby.id.len(), 10);
        assert_eq!(default_lobby.guessing_time, 80);
        assert_eq!(default_lobby.started_at, None);
        assert_eq!(
            default_lobby.created_at.date().day(),
            chrono::Utc::now().naive_utc().date().day()
        );
    }
}
