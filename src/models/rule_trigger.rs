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
pub struct RuleTrigger {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The type of thing this trigger responds to. A limited set is possible
    #[serde(rename = "type")]
    pub _type: Type,
    /// The accompanying value the trigger responds to. This value is often mandatory, but this depends on the trigger.
    #[serde(rename = "value")]
    pub value: String,
    /// Order of the trigger
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// If the trigger is active.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// When true, other triggers will not be checked if this trigger was triggered.
    #[serde(rename = "stop_processing", skip_serializing_if = "Option::is_none")]
    pub stop_processing: Option<bool>,
}

impl RuleTrigger {
    pub fn new(_type: Type, value: String) -> RuleTrigger {
        RuleTrigger {
            id: None,
            created_at: None,
            updated_at: None,
            _type,
            value,
            order: None,
            active: None,
            stop_processing: None,
        }
    }
}

/// The type of thing this trigger responds to. A limited set is possible
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "from_account_starts")]
    FromAccountStarts,
    #[serde(rename = "from_account_ends")]
    FromAccountEnds,
    #[serde(rename = "from_account_is")]
    FromAccountIs,
    #[serde(rename = "from_account_contains")]
    FromAccountContains,
    #[serde(rename = "to_account_starts")]
    ToAccountStarts,
    #[serde(rename = "to_account_ends")]
    ToAccountEnds,
    #[serde(rename = "to_account_is")]
    ToAccountIs,
    #[serde(rename = "to_account_contains")]
    ToAccountContains,
    #[serde(rename = "amount_less")]
    AmountLess,
    #[serde(rename = "amount_exactly")]
    AmountExactly,
    #[serde(rename = "amount_more")]
    AmountMore,
    #[serde(rename = "description_starts")]
    DescriptionStarts,
    #[serde(rename = "description_ends")]
    DescriptionEnds,
    #[serde(rename = "description_contains")]
    DescriptionContains,
    #[serde(rename = "description_is")]
    DescriptionIs,
    #[serde(rename = "transaction_type")]
    TransactionType,
    #[serde(rename = "category_is")]
    CategoryIs,
    #[serde(rename = "budget_is")]
    BudgetIs,
    #[serde(rename = "tag_is")]
    TagIs,
    #[serde(rename = "currency_is")]
    CurrencyIs,
    #[serde(rename = "has_attachments")]
    HasAttachments,
    #[serde(rename = "has_no_category")]
    HasNoCategory,
    #[serde(rename = "has_any_category")]
    HasAnyCategory,
    #[serde(rename = "has_no_budget")]
    HasNoBudget,
    #[serde(rename = "has_any_budget")]
    HasAnyBudget,
    #[serde(rename = "has_no_tag")]
    HasNoTag,
    #[serde(rename = "has_any_tag")]
    HasAnyTag,
    #[serde(rename = "notes_contain")]
    NotesContain,
    #[serde(rename = "notes_start")]
    NotesStart,
    #[serde(rename = "notes_end")]
    NotesEnd,
    #[serde(rename = "notes_are")]
    NotesAre,
    #[serde(rename = "no_notes")]
    NoNotes,
    #[serde(rename = "any_notes")]
    AnyNotes,
}

