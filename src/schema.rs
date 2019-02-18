table! {
    bonus_accounts (id) {
        id -> Int4,
        user_id -> Int4,
        bonuses -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    bonus_logs (id) {
        id -> Int4,
        bonus_account_id -> Int4,
        user_order_id -> Int4,
        bonuses -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_orders (id) {
        id -> Int4,
        user_id -> Int4,
        product -> Varchar,
        price -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(bonus_accounts -> users (user_id));
joinable!(bonus_logs -> bonus_accounts (bonus_account_id));
joinable!(bonus_logs -> user_orders (user_order_id));
joinable!(user_orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    bonus_accounts,
    bonus_logs,
    user_orders,
    users,
);
