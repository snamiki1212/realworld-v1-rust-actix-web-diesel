table! {
    tags (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        bio -> Text,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tags,
    users,
);
