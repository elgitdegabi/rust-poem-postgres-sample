use log::info;

use crate::model::user::{User, UserUpsert};
use crate::repository::*;

pub fn get_users() -> Vec<User> {
    info!("user_service - get_users - executed");
    user_repository::get_users()
}

pub fn get_user(user_id: i64) -> User {
    info!(
        "user_service - get_user - executed for user id: {}",
        user_id
    );
    user_repository::get_user(user_id)
}

pub fn add_user(user: UserUpsert) -> User {
    info!("user_service - add_user - executed for user: {:?}", user);
    user_repository::add_user(user)
}

pub fn update_user(user_id: i64, user: UserUpsert) -> User {
    info!(
        "user_service - update_user - executed for user id: {} - user: {:?}",
        user_id, user
    );
    user_repository::update_user(user_id, user)
}

pub fn delete_user(user_id: i64) -> String {
    info!(
        "user_service - delete_user - executed for user id: {}",
        user_id
    );
    user_repository::delete_user(user_id)
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes get_users with valid parameters
     * Expectation:
     * A list of users should be retrieved
     */
    #[test]
    fn when_get_users_with_valid_parameters_should_return_user_list() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes get_user with valid parameters
     * Expectation:
     * An user should be retrieved
     */
    #[test]
    fn when_get_user_with_valid_parameters_should_return_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes add_user with valid parameters
     * Expectation:
     * An user should be created
     */
    #[test]
    fn when_add_user_with_valid_parameters_should_create_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes update_user with valid parameters
     * Expectation:
     * An user should be updated
     */
    #[test]
    fn when_update_user_with_valid_parameters_should_update_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes delete_user with valid parameters
     * Expectation:
     * An user should be deleted
     */
    #[test]
    fn when_delete_user_with_valid_parameters_should_delete_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
