use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Schema model import
use super::super::schema::users;
use super::super::schema::users::dsl::*;

// STRUCT MODELS

// Struct model
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct ModelUsers {
    pub id_users: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub status: bool,
}

// Struct model insertable
#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct StructNewUsers<'a> {
    pub id_users: &'a Uuid,
    pub first_name: &'a String,
    pub last_name: &'a String,
    pub email: &'a String,
    pub status: &'a bool,
}

// Struct model handler
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructHandlerUsers {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub status: bool,
}

// MODELS
impl ModelUsers {
    pub fn get_users(conn: &mut PgConnection) -> Result<Vec<ModelUsers>, diesel::result::Error> {
        let all_users = users.load::<ModelUsers>(conn);
        all_users
    }

    pub fn new_user(
        conn: &mut PgConnection,
        user: &StructHandlerUsers,
    ) -> Result<ModelUsers, diesel::result::Error> {
        let new_user = StructNewUsers {
            id_users: &Uuid::new_v4(),
            first_name: &user.first_name,
            last_name: &user.last_name,
            email: &user.email,
            status: &user.status,
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<ModelUsers>(conn)
    }
}
