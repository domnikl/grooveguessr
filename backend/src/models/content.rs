use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_schema::contents;

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
#[diesel(table_name = contents)]
pub struct Contents {
    pub id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}
