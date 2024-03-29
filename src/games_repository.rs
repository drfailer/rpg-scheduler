use crate::models::Game;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

pub fn insert_game(conn: &mut SqliteConnection, game_name: String) -> diesel::QueryResult<Game> {
    use crate::schema::games::dsl::*;

    // Create insertion model
    let uid = format!("{}", uuid::Uuid::new_v4());
    let game = Game {
        id: uid.to_string(),
        schedule_time: "".to_string(),
        end_time: "".to_string(),
        name: game_name,
        gm_discord_name: "1234".to_string(),
        gm_discord_id: "1234".to_string(),
    };

    // normal diesel operations
    diesel::insert_into(games)
        .values(&game)
        .execute(conn)
        .expect("Error inserting person");
    games.find(uid).first(conn)
}

pub fn get_game(conn: &mut SqliteConnection, game_id: String) -> diesel::QueryResult<Game> {
    use crate::schema::games::dsl::*;
    games.find(game_id).first(conn)
}

pub fn delete_game(conn: &mut SqliteConnection, game_id: String) -> diesel::QueryResult<usize> {
    use crate::schema::games::dsl::*;
    let delete_row = diesel::delete(games.filter(id.eq(game_id))).execute(conn)?;
    Ok(delete_row)
}
