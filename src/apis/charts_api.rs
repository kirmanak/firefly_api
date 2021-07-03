/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_chart_ab_overview`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChartAbOverviewError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_chart_account_expense`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChartAccountExpenseError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_chart_account_overview`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChartAccountOverviewError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_chart_account_revenue`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChartAccountRevenueError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_chart_category_overview`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChartCategoryOverviewError {
    UnknownValue(serde_json::Value),
}


/// This endpoint returns the data required to generate a pie chart for the available budget. 
pub async fn get_chart_ab_overview(configuration: &configuration::Configuration, id: i32, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error<GetChartAbOverviewError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/chart/ab/overview/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("start", &start.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end", &end.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetChartAbOverviewError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint returns the data required to generate a chart that shows the user how much they've spent on their expense accounts. 
pub async fn get_chart_account_expense(configuration: &configuration::Configuration, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error<GetChartAccountExpenseError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/chart/account/expense", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("start", &start.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end", &end.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetChartAccountExpenseError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint returns the data required to generate a chart with basic asset account balance information. 
pub async fn get_chart_account_overview(configuration: &configuration::Configuration, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error<GetChartAccountOverviewError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/chart/account/overview", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("start", &start.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end", &end.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetChartAccountOverviewError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint returns the data required to generate a chart that shows the user how much they've earned from their revenue accounts. 
pub async fn get_chart_account_revenue(configuration: &configuration::Configuration, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error<GetChartAccountRevenueError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/chart/account/revenue", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("start", &start.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end", &end.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetChartAccountRevenueError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint returns the data required to generate a bar chart for the expenses and incomes on the users categories. 
pub async fn get_chart_category_overview(configuration: &configuration::Configuration, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error<GetChartCategoryOverviewError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/chart/category/overview", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("start", &start.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end", &end.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetChartCategoryOverviewError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
