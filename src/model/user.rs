use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use schema::user_data;
use serde::{Deserialize, Serialize};

use crate::schema;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_data)]
pub struct User {
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_alias: Option<String>,
    pub user_address: Option<String>,
    pub user_create_ts: Option<NaiveDateTime>,
    pub user_update_ts: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_data)]
pub struct UserUpsert {
    pub user_name: Option<String>,
    pub user_alias: Option<String>,
    pub user_address: Option<String>,
    pub user_create_ts: Option<NaiveDateTime>,
    pub user_update_ts: Option<NaiveDateTime>,
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes User struct validation with valid values
     * Expectation:
     * The struct with proper values should be created
     */
    #[test]
    fn when_create_user_struct_should_retrieve_struct_values() {
        let user_id: i64 = 1;
        let user_name = "some user name";
        let user_alias = "some user alias";
        let user_address = "some user address";
        let user_create_ts = NaiveDateTime::from_timestamp_millis(100);
        let user_update_ts = NaiveDateTime::from_timestamp_millis(900);

        let user = User {
            user_id,
            user_name: Option::from(String::from(user_name)),
            user_alias: Option::from(String::from(user_alias)),
            user_address: Option::from(String::from(user_address)),
            user_create_ts,
            user_update_ts,
        };

        assert_eq!(user_id, user.user_id);
        assert_eq!(user_name, user.user_name.unwrap());
        assert_eq!(user_alias, user.user_alias.unwrap());
        assert_eq!(user_address, user.user_address.unwrap());
        assert_eq!(user_create_ts, user.user_create_ts);
        assert_eq!(user_update_ts, user.user_update_ts);
    }

    /**
     * Scenario:
     * Executes UserUpsert struct validation with valid values
     * Expectation:
     * The struct with proper values should be created
     */
    #[test]
    fn when_create_user_upsert_struct_should_retrieve_struct_values() {
        let user_name = "some user name";
        let user_alias = "some user alias";
        let user_address = "some user address";
        let user_create_ts = NaiveDateTime::from_timestamp_millis(100);
        let user_update_ts = NaiveDateTime::from_timestamp_millis(900);

        let user = UserUpsert {
            user_name: Option::from(String::from(user_name)),
            user_alias: Option::from(String::from(user_alias)),
            user_address: Option::from(String::from(user_address)),
            user_create_ts,
            user_update_ts,
        };

        assert_eq!(user_name, user.user_name.unwrap());
        assert_eq!(user_alias, user.user_alias.unwrap());
        assert_eq!(user_address, user.user_address.unwrap());
        assert_eq!(user_create_ts, user.user_create_ts);
        assert_eq!(user_update_ts, user.user_update_ts);
    }
}
