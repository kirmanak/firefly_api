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
pub struct BillPaidDates {
    /// Transaction group ID of the paid bill.
    #[serde(rename = "transaction_group_id", skip_serializing_if = "Option::is_none")]
    pub transaction_group_id: Option<i32>,
    /// Transaction journal ID of the paid bill.
    #[serde(rename = "transaction_journal_id", skip_serializing_if = "Option::is_none")]
    pub transaction_journal_id: Option<i32>,
    /// Date the bill was paid.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl BillPaidDates {
    pub fn new() -> BillPaidDates {
        BillPaidDates {
            transaction_group_id: None,
            transaction_journal_id: None,
            date: None,
        }
    }
}


