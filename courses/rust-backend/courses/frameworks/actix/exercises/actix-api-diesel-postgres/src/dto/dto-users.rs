use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
// use uuid::Uuid;

// Schema model import
use super::super::schema::users;
use super::super::schema::users::dsl::*;


// Models
#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct StructUsers<'a> {
    pub id_users &'a u4,
}
