use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db_schema::users;

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
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}
