table! {
    posts (id) {
        id -> Bigint,
        user_id -> Bigint,
        body -> Text,
    }
}

table! {
    users (id) {
        id -> Bigint,
        name -> Text,
        email -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
