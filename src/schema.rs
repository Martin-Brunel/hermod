// @generated automatically by Diesel CLI.

diesel::table! {
    brand (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        interval -> Nullable<Integer>,
    }
}

diesel::table! {
    message (id) {
        id -> Integer,
        user_id -> Integer,
        data -> Text,
        is_read -> Tinyint,
        session_id -> Integer,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    session (id) {
        id -> Integer,
        user_id -> Integer,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        roles -> Varchar,
        brand_id -> Integer,
        firstname -> Varchar,
        lastname -> Varchar,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::joinable!(message -> session (session_id));
diesel::joinable!(message -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    brand,
    message,
    session,
    user,
);
