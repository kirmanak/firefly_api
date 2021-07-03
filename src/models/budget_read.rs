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
pub struct BudgetRead {
    /// Immutable value
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::Budget>,
}

impl BudgetRead {
    pub fn new(_type: String, id: String, attributes: crate::models::Budget) -> BudgetRead {
        BudgetRead {
            _type,
            id,
            attributes: Box::new(attributes),
        }
    }
}


