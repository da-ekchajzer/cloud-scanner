/*
 * BOAVIZTAPI - DEMO
 *
 * # 🎯 Retrieving the impacts of digital elements This is a quick demo, to see full documentation [click here](http://api.boavizta.org)  ## ➡️Server router  ### Server routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Cloud router  ### Cloud routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Component router  ### Component routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |          | |   ADP  |        X       |          | |   PE   |        X       |          | 
 *
 * The version of the OpenAPI document: 0.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`case_impact_bottom_up_v1_component_case_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CaseImpactBottomUpV1ComponentCasePostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cpu_impact_bottom_up_v1_component_cpu_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CpuImpactBottomUpV1ComponentCpuPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disk_impact_bottom_up_v1_component_hdd_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiskImpactBottomUpV1ComponentHddPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disk_impact_bottom_up_v1_component_ssd_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiskImpactBottomUpV1ComponentSsdPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`motherboard_impact_bottom_up_v1_component_motherboard_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MotherboardImpactBottomUpV1ComponentMotherboardPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`power_supply_impact_bottom_up_v1_component_power_supply_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PowerSupplyImpactBottomUpV1ComponentPowerSupplyPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ram_impact_bottom_up_v1_component_ram_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RamImpactBottomUpV1ComponentRamPostError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// # ✔️Case impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure The impacts values are set by default
pub async fn case_impact_bottom_up_v1_component_case_post(configuration: &configuration::Configuration, verbose: Option<bool>, case: Option<crate::models::Case>) -> Result<serde_json::Value, Error<CaseImpactBottomUpV1ComponentCasePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/case", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&case);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CaseImpactBottomUpV1ComponentCasePostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️CPU impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>cpu<sub>manuf<sub><em>criteria</em></sub></sub> = ( cpu<sub>core<sub>units</sub></sub> x cpu<sub>diesize</sub> + 0,491 ) x cpu<sub>manuf_die<sub><em>criteria</em></sub></sub> + cpu<sub>manuf_base<sub><em>criteria</em></sub></sub></h3> 
pub async fn cpu_impact_bottom_up_v1_component_cpu_post(configuration: &configuration::Configuration, verbose: Option<bool>, cpu: Option<crate::models::Cpu>) -> Result<serde_json::Value, Error<CpuImpactBottomUpV1ComponentCpuPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/cpu", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&cpu);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CpuImpactBottomUpV1ComponentCpuPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️HDD impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure The impacts values are set by default
pub async fn disk_impact_bottom_up_v1_component_hdd_post(configuration: &configuration::Configuration, verbose: Option<bool>, disk: Option<crate::models::Disk>) -> Result<serde_json::Value, Error<DiskImpactBottomUpV1ComponentHddPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/hdd", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&disk);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DiskImpactBottomUpV1ComponentHddPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️SSD impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>ssd<sub>manuf<sub><em>criteria</em></sub></sub> =  ( ssd<sub>size</sub> ssd<sub>density</sub> ) x ssd<sub>manuf_die<sub><em>criteria</em></sub></sub> + ssd<sub>manuf_base<sub><em>criteria</em></sub></sub></h3> 
pub async fn disk_impact_bottom_up_v1_component_ssd_post(configuration: &configuration::Configuration, verbose: Option<bool>, disk: Option<crate::models::Disk>) -> Result<serde_json::Value, Error<DiskImpactBottomUpV1ComponentSsdPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/ssd", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&disk);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DiskImpactBottomUpV1ComponentSsdPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️Motherboard impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure The impacts values are set by default
pub async fn motherboard_impact_bottom_up_v1_component_motherboard_post(configuration: &configuration::Configuration, verbose: Option<bool>, mother_board: Option<crate::models::MotherBoard>) -> Result<serde_json::Value, Error<MotherboardImpactBottomUpV1ComponentMotherboardPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/motherboard", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&mother_board);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MotherboardImpactBottomUpV1ComponentMotherboardPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️Power supply impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>psu<sub>manuf<sub><em>criteria</em></sub></sub> = psu<sub>unit<sub>weight</sub></sub> x psu<sub>manuf_weight<sub><em>criteria</em></sub></sub></h3> 
pub async fn power_supply_impact_bottom_up_v1_component_power_supply_post(configuration: &configuration::Configuration, verbose: Option<bool>, power_supply: Option<crate::models::PowerSupply>) -> Result<serde_json::Value, Error<PowerSupplyImpactBottomUpV1ComponentPowerSupplyPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/power_supply", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&power_supply);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PowerSupplyImpactBottomUpV1ComponentPowerSupplyPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # ✔️RAM impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>ram<sub>manuf<sub><em>criteria</em></sub></sub> =( ram<sub>size</sub> / ram<sub>density</sub> ) x ram<sub>manuf_die<sub><em>criteria</em></sub></sub> + ram<sub>manuf_base<sub><em>criteria</em></sub></sub> </h3> 
pub async fn ram_impact_bottom_up_v1_component_ram_post(configuration: &configuration::Configuration, verbose: Option<bool>, ram: Option<crate::models::Ram>) -> Result<serde_json::Value, Error<RamImpactBottomUpV1ComponentRamPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/component/ram", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = verbose {
        local_var_req_builder = local_var_req_builder.query(&[("verbose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&ram);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RamImpactBottomUpV1ComponentRamPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

