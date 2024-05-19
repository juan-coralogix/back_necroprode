use diesel::prelude::*;
use diesel::sql_types::{Integer, Timestamp, Varchar};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Necroprode {
    pub id: i32,
    pub name: String,
    pub creator_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "necroprodes"]
pub struct NewNecroprode<'a> {
    pub name: &'a str,
    pub creator_id: i32,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct NecroprodeMember {
    pub necroprode_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "necroprode_members"]
pub struct NewNecroprodeMember {
    pub necroprode_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Selection {
    pub id: i32,
    pub necroprode_id: i32,
    pub user_id: i32,
    pub celebrity_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "selections"]
pub struct NewSelection<'a> {
    pub necroprode_id: i32,
    pub user_id: i32,
    pub celebrity_name: &'a str,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}