/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableBudget {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Use either currency_id or currency_code.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<i32>,
    /// Use either currency_id or currency_code.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    #[serde(rename = "amount")]
    pub amount: String,
    /// Start date of the available budget.
    #[serde(rename = "start")]
    pub start: String,
    /// End date of the available budget.
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "spent_in_budgets", skip_serializing_if = "Option::is_none")]
    pub spent_in_budgets: Option<Vec<crate::models::BudgetSpent>>,
    #[serde(rename = "spent_outside_budget", skip_serializing_if = "Option::is_none")]
    pub spent_outside_budget: Option<Vec<crate::models::BudgetSpent>>,
}

impl AvailableBudget {
    pub fn new(amount: String, start: String, end: String) -> AvailableBudget {
        AvailableBudget {
            created_at: None,
            updated_at: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
            amount,
            start,
            end,
            spent_in_budgets: None,
            spent_outside_budget: None,
        }
    }
}

