table! {
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

table! {
    comments (id) {
        id -> Uuid,
        article_id -> Uuid,
        author_id -> Uuid,
        body -> Text,
        create_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    favorites (id) {
        id -> Uuid,
        article_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    follows (follower_id, followee_id) {
        followee_id -> Uuid,
        follower_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        article_id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
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

joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (author_id));
joinable!(favorites -> articles (article_id));
joinable!(favorites -> users (user_id));
joinable!(tags -> articles (article_id));

allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    favorites,
    follows,
    tags,
    users,
);
