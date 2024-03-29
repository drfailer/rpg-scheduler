// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Text,
        discord_name -> Text,
        discord_id -> Text,
        character_name -> Text,
        game_id -> Text,
    }
}

diesel::table! {
    games (id) {
        id -> Text,
        schedule_time -> Timestamp,
        end_time -> Timestamp,
        name -> Text,
        gm_discord_name -> Text,
        gm_discord_id -> Text,
    }
}

diesel::joinable!(characters -> games (id));

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    games,
);
