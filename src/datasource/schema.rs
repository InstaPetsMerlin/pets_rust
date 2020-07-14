table! {
    pets (id) {
        id -> Int4,
        name -> Varchar,
        age -> Nullable<Int2>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        text -> Varchar,
        image -> Varchar,
        date -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    pets,
    posts,
    users,
);
