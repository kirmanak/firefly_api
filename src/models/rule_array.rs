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
pub struct RuleArray {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::RuleRead>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::Meta>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::PageLink>,
}

impl RuleArray {
    pub fn new(data: Vec<crate::models::RuleRead>, meta: crate::models::Meta, links: crate::models::PageLink) -> RuleArray {
        RuleArray {
            data,
            meta: Box::new(meta),
            links: Box::new(links),
        }
    }
}


