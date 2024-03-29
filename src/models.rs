use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::games)]
pub struct Game {
    pub id: String,
    pub schedule_time: String,
    pub end_time: String,
    pub name: String,
    pub gm_discord_name: String,
    pub gm_discord_id: String
}

#[derive(Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::characters)]
pub struct Character {
    pub id: String,
    pub discord_name: String,
    pub discord_id: String,
    pub character_name: String,
    pub game_id: String // foreign key
}
