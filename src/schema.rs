table! {
    urls (id) {
        id -> Uuid,
        slug -> Varchar,
        url -> Varchar,
        visits -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
