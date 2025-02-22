use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

use crate::{cloud_resource::CloudResource, impact_provider::CloudResourceWithImpacts};

///  Statistics about program execution
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionStatistics {
    pub inventory_duration: Duration,
    pub impact_duration: Duration,
    pub total_duration: Duration,
}

impl fmt::Display for ExecutionStatistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Inventory
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub resources: Vec<CloudResource>,
    pub execution_statistics: ExecutionStatistics,
}

/// Impacts results
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesWithImpacts {
    pub impacts: Vec<CloudResourceWithImpacts>,
    pub execution_statistics: ExecutionStatistics,
}
