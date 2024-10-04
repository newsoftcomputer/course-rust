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
pub struct StructUsers {
    pub id_users: u8,
    pub fisrt_name: String,
    pub last_name: String,
    pub email: String,
    pub status: Boolean,
}

// Struct model insertable
#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct StructNewUsers<'a> {
    pub id_users: &'a u8,
    pub fisrt_name: &'a String,
    pub last_name: &'a String,
    pub email: &'a String,
    pub status: &'a Boolean,
}

// Struct model handler
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructHandlerUsers {
    pub fisrt_name: String,
    pub last_name: String,
    pub email: String,
    pub status: Boolean,
}

// MODELS
impl ModelUsers {
    pub fn get_users(conn: &mut PgConnection) -> Result<Vec<StructUsers>, diesel::result::Error> {
        let all_users = users.load::<StructUsers>(conn);
        all_users
    }

    pub fn new_user(
        conn: &mut PgConnection,
        user: &StructHandlerUsers,
    ) -> Result<StructUsers, diesel::result::Error> {
        let new_user = StructNewUsers {
            id_users: &user.id_users,
            fisrt_name: &user.fisrt_name,
            last_name: &user.last_name,
            email: &user.email,
            status: &user.status,
        };

        diesel::inser_into(users::table)
            .values(new_users)
            .get_result::<StructUsers>(conn)
    }
}
