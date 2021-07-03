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
pub struct LinkType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "inward")]
    pub inward: String,
    #[serde(rename = "outward")]
    pub outward: String,
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
}

impl LinkType {
    pub fn new(name: String, inward: String, outward: String) -> LinkType {
        LinkType {
            name,
            inward,
            outward,
            editable: None,
        }
    }
}


