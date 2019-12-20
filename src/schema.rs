table! {
    urls (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        slug -> Varchar,
        url -> Varchar,
        visits -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        scope -> TokenScope,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        display_name -> Varchar,
        email -> Text,
        email_verified -> Bool,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(urls -> users (user_id));

allow_tables_to_appear_in_same_query!(
    urls,
    user_tokens,
    users,
);
