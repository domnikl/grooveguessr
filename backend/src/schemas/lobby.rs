use async_graphql::SimpleObject;
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::db_schema::lobbies;

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
)]
#[diesel(table_name = lobbies)]
pub struct Lobby {
    pub id: String,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub guessing_time: i16,
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
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use crate::schemas::lobby;

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
