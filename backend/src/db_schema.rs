// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Uuid,
        #[sql_name = "type"]
        #[max_length = 70]
        type_ -> Varchar,
        #[max_length = 255]
        data -> Varchar,
        user_id -> Uuid,
        #[max_length = 10]
        lobby_id -> Bpchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    lobbies (id) {
        #[max_length = 10]
        id -> Bpchar,
        started_at -> Nullable<Timestamptz>,
        guessing_time -> Int2,
        host_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
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

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    lobbies,
    users,
);
