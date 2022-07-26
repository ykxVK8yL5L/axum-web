table! {
    login_history (id) {
        id -> Integer,
        user_id -> Integer,
        login_timestamp -> Timestamp,
    }
}

table! {
    songs (id) {
        id -> Integer,
        name -> Text,
        artist -> Text,
        filename -> Nullable<Text>,
    }
}


table! {
    videos (id) {
        id -> Integer,
        name -> Text,
        title -> Text,
        cid -> Text,
        size -> Nullable<Integer>,
        img -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        login_session -> Nullable<Text>,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    login_history,
    songs,
    videos,
    users,
);
