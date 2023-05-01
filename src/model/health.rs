//extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub status: String,
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes Health struct validation with valid values
     * Expectation:
     * The struct with proper values should be created
     */
    #[test]
    fn when_create_health_struct_should_retrieve_struct_values() {
        let status_value = "some_value";
        let health = Health {
            status: String::from(status_value),
        };
        assert_eq!(status_value, health.status);
    }
}
