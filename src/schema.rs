// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        article_id -> Uuid,
        author_id -> Uuid,
        body -> Text,
        create_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    favorites (id) {
        id -> Uuid,
        article_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    follows (follower_id, followee_id) {
        followee_id -> Uuid,
        follower_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        article_id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        password -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(articles -> users (author_id));
diesel::joinable!(comments -> articles (article_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(favorites -> articles (article_id));
diesel::joinable!(favorites -> users (user_id));
diesel::joinable!(tags -> articles (article_id));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    favorites,
    follows,
    tags,
    users,
);
