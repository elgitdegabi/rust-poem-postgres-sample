use poem::{get, handler, post, web::Json, Request, Route};
use serde_json::json;

use crate::config::constants::*;
use crate::model::health::Health;
use crate::model::user::UserUpsert;
use crate::service::user_service::*;

pub fn config_endpoints() -> Route {
    #[handler]
    async fn map_health() -> String {
        format!(
            "{}",
            json!(Health {
                status: String::from(SERVER_RUNNING_STATUS)
            })
        )
    }

    #[handler]
    async fn map_get_users() -> String {
        format!("{}", json!(get_users()))
    }

    #[handler]
    async fn map_get_user(request: &Request) -> String {
        let id = request.path_params::<i64>().unwrap();
        format!("{}", json!(get_user(id)))
    }

    #[handler]
    async fn map_add_user(user: Json<UserUpsert>) -> String {
        format!("{}", json!(add_user(user.0)))
    }

    #[handler]
    async fn map_update_user(request: &Request, user: Json<UserUpsert>) -> String {
        let id = request.path_params::<i64>().unwrap();
        format!("{}", json!(update_user(id, user.0)))
    }

    #[handler]
    async fn map_delete_user(request: &Request) -> String {
        let id = request.path_params::<i64>().unwrap();
        format!("{}", json!(delete_user(id)))
    }

    Route::new()
        .at("/health", get(map_health))
        .at("/users", get(map_get_users))
        .at("/user/:userid", get(map_get_user))
        .at("/user/add", post(map_add_user))
        .at("/user/update/:userid", post(map_update_user))
        .at("/user/delete/:userid", post(map_delete_user))
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes config_endpoints with valid API server
     * Expectation:
     * A set of endpoints should be created
     */
    #[test]
    fn when_config_endpoints_with_api_server_should_add_endpoints() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
