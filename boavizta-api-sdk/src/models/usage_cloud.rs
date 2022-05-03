/*
 * BOAVIZTAPI - DEMO
 *
 * # 🎯 Retrieving the impacts of digital elements This is a quick demo, to see full documentation [click here](http://api.boavizta.org)  ## ➡️Server router  ### Server routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Cloud router  ### Cloud routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Component router  ### Component routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |          | |   ADP  |        X       |          | |   PE   |        X       |          | 
 *
 * The version of the OpenAPI document: 0.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsageCloud {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "TYPE", skip_serializing_if = "Option::is_none")]
    pub TYPE: Option<String>,
    #[serde(rename = "years_use_time", skip_serializing_if = "Option::is_none")]
    pub years_use_time: Option<f32>,
    #[serde(rename = "days_use_time", skip_serializing_if = "Option::is_none")]
    pub days_use_time: Option<f32>,
    #[serde(rename = "hours_use_time", skip_serializing_if = "Option::is_none")]
    pub hours_use_time: Option<f32>,
    #[serde(rename = "hours_electrical_consumption", skip_serializing_if = "Option::is_none")]
    pub hours_electrical_consumption: Option<f32>,
    #[serde(rename = "usage_location", skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[serde(rename = "gwp_factor", skip_serializing_if = "Option::is_none")]
    pub gwp_factor: Option<f32>,
    #[serde(rename = "pe_factor", skip_serializing_if = "Option::is_none")]
    pub pe_factor: Option<f32>,
    #[serde(rename = "adp_factor", skip_serializing_if = "Option::is_none")]
    pub adp_factor: Option<f32>,
    #[serde(rename = "max_power", skip_serializing_if = "Option::is_none")]
    pub max_power: Option<f32>,
    #[serde(rename = "workload", skip_serializing_if = "Option::is_none")]
    pub workload: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, f32>>>,
    #[serde(rename = "instance_per_server", skip_serializing_if = "Option::is_none")]
    pub instance_per_server: Option<f32>,
}

impl UsageCloud {
    pub fn new() -> UsageCloud {
        UsageCloud {
            hash: None,
            TYPE: None,
            years_use_time: None,
            days_use_time: None,
            hours_use_time: None,
            hours_electrical_consumption: None,
            usage_location: None,
            gwp_factor: None,
            pe_factor: None,
            adp_factor: None,
            max_power: None,
            workload: None,
            instance_per_server: None,
        }
    }
}


