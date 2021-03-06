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
pub struct ObjectLink0 {
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ObjectLink0 {
    pub fn new() -> ObjectLink0 {
        ObjectLink0 {
            rel: None,
            uri: None,
        }
    }
}


