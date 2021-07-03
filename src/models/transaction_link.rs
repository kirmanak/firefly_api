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
pub struct TransactionLink {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The link type ID to use. You can also use the link_type_name field.
    #[serde(rename = "link_type_id")]
    pub link_type_id: i32,
    /// The link type name to use. You can also use the link_type_id field.
    #[serde(rename = "link_type_name", skip_serializing_if = "Option::is_none")]
    pub link_type_name: Option<String>,
    /// The inward transaction transaction_journal_id for the link. This becomes the 'is paid by' transaction of the set.
    #[serde(rename = "inward_id")]
    pub inward_id: i32,
    /// The outward transaction transaction_journal_id for the link. This becomes the 'pays for' transaction of the set.
    #[serde(rename = "outward_id")]
    pub outward_id: i32,
    /// Optional. Some notes.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl TransactionLink {
    pub fn new(link_type_id: i32, inward_id: i32, outward_id: i32) -> TransactionLink {
        TransactionLink {
            created_at: None,
            updated_at: None,
            link_type_id,
            link_type_name: None,
            inward_id,
            outward_id,
            notes: None,
        }
    }
}


