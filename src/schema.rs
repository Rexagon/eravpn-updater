table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    releases (id) {
        id -> Int4,
        version_major -> Int4,
        version_minor -> Int4,
        version_patch -> Int4,
        creation_date -> Timestamp,
        active -> Bool,
        description -> Nullable<Text>,
        changelog -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    releases,
);
