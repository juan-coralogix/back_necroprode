// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    necroprodes (id) {
        id -> Int4,
        name -> Varchar,
        creator_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    necroprode_members (necroprode_id, user_id) {
        necroprode_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    selections (id) {
        id -> Int4,
        necroprode_id -> Int4,
        user_id -> Int4,
        celebrity_name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(necroprode_members -> necroprodes (necroprode_id));
diesel::joinable!(necroprode_members -> users (user_id));
diesel::joinable!(selections -> necroprodes (necroprode_id));
diesel::joinable!(selections -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    necroprodes,
    necroprode_members,
    selections,
);