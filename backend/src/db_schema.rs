// @generated automatically by Diesel CLI.

diesel::table! {
    contents (lobby_id, user_id) {
        #[max_length = 10]
        lobby_id -> Bpchar,
        #[max_length = 100]
        user_id -> Varchar,
        #[sql_name = "type"]
        #[max_length = 70]
        type_ -> Varchar,
        #[max_length = 255]
        data -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    lobbies (id) {
        #[max_length = 10]
        id -> Bpchar,
        started_at -> Nullable<Timestamptz>,
        guessing_time -> Int2,
        #[max_length = 100]
        host_id -> Varchar,
        sequence -> Nullable<Text>,
        #[max_length = 100]
        current_user_id -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    lobbies_players (lobby_id, player_id) {
        #[max_length = 10]
        lobby_id -> Bpchar,
        #[max_length = 100]
        player_id -> Varchar,
        is_ready -> Bool,
        guesses -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 100]
        id -> Varchar,
        #[max_length = 70]
        email -> Varchar,
        #[max_length = 70]
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(contents -> lobbies (lobby_id));
diesel::joinable!(contents -> users (user_id));
diesel::joinable!(lobbies -> users (host_id));
diesel::joinable!(lobbies_players -> lobbies (lobby_id));
diesel::joinable!(lobbies_players -> users (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    lobbies,
    lobbies_players,
    users,
);
