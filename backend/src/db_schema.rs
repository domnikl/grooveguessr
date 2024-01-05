// @generated automatically by Diesel CLI.

diesel::table! {
    lobbies (id) {
        #[max_length = 10]
        id -> Bpchar,
        started_at -> Nullable<Timestamptz>,
        guessing_time -> Int2,
        created_at -> Timestamptz,
    }
}
