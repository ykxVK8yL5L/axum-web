table! {
    gateways (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
    }
}

table! {
    login_history (id) {
        id -> Integer,
        user_id -> Integer,
        login_timestamp -> Timestamp,
    }
}

table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
        desc -> Nullable<Text>,
        hidden-> Bool,
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
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        login_session -> Nullable<Text>,
    }
}

table! {
    videos (id) {
        id -> Integer,
        name -> Text,
        title -> Text,
        cid -> Text,
        size -> Nullable<Text>,
        img -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    gateways,
    login_history,
    settings,
    songs,
    users,
    videos,
);
