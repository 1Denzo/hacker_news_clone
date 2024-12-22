// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        comment -> Varchar,
        post_id -> Int4,
        user_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        link -> Nullable<Varchar>,
        author -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(posts -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
