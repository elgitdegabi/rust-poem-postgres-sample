// @generated automatically by Diesel CLI.

diesel::table! {
    user_data (user_id) {
        user_id -> BigInt,
        user_name -> Nullable<Text>,
        user_alias -> Nullable<Text>,
        user_address -> Nullable<Text>,
        user_create_ts -> Nullable<Timestamptz>,
        user_update_ts -> Nullable<Timestamptz>,
    }
}
