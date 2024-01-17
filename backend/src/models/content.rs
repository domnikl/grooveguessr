use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db_schema::contents;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    SimpleObject,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = contents)]
#[diesel(primary_key(lobby_id, user_id))]
pub struct Contents {
    pub lobby_id: String,
    pub user_id: String,
    pub type_: String,
    pub data: String,
    pub created_at: chrono::NaiveDateTime,
}
