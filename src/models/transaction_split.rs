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
pub struct TransactionSplit {
    /// User ID
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// ID of the underlying transaction journal. Each transaction consists of a transaction group (see the top ID) and one or more journals making up the splits of the transaction. 
    #[serde(rename = "transaction_journal_id", skip_serializing_if = "Option::is_none")]
    pub transaction_journal_id: Option<i32>,
    /// Type of transaction.
    #[serde(rename = "type")]
    pub _type: Type,
    /// Date of the transaction
    #[serde(rename = "date")]
    pub date: String,
    /// Amount of the transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Description of the transaction.
    #[serde(rename = "description")]
    pub description: String,
    /// Order of this entry in the list of transactions.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// Currency ID. Default is the source account's currency, or the user's default currency. Can be used instead of currency_code.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Currency code. Default is the source account's currency, or the user's default currency. Can be used instead of currency_id.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "currency_name", skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    /// Number of decimals used in this currency.
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    /// The amount in a foreign currency.
    #[serde(rename = "foreign_amount", skip_serializing_if = "Option::is_none")]
    pub foreign_amount: Option<String>,
    /// Currency ID of the foreign currency. Default is null. Is required when you submit a foreign amount.
    #[serde(rename = "foreign_currency_id", skip_serializing_if = "Option::is_none")]
    pub foreign_currency_id: Option<String>,
    /// Currency code of the foreign currency. Default is NULL. Can be used instead of the foreign_currency_id, but this or the ID is required when submitting a foreign amount.
    #[serde(rename = "foreign_currency_code", skip_serializing_if = "Option::is_none")]
    pub foreign_currency_code: Option<String>,
    #[serde(rename = "foreign_currency_symbol", skip_serializing_if = "Option::is_none")]
    pub foreign_currency_symbol: Option<String>,
    /// Number of decimals in the currency
    #[serde(rename = "foreign_currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub foreign_currency_decimal_places: Option<i32>,
    /// The budget ID for this transaction.
    #[serde(rename = "budget_id", skip_serializing_if = "Option::is_none")]
    pub budget_id: Option<i32>,
    /// The name of the budget to be used. If the budget name is unknown, the ID will be used or the value will be ignored.
    #[serde(rename = "budget_name", skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    /// The category ID for this transaction.
    #[serde(rename = "category_id", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// The name of the category to be used. If the category is unknown, it will be created. If the ID and the name point to different categories, the ID overrules the name.
    #[serde(rename = "category_name", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    /// ID of the source account. For a withdrawal or a transfer, this must always be an asset account. For deposits, this must be a revenue account.
    #[serde(rename = "source_id")]
    pub source_id: Option<String>,
    /// Name of the source account. For a withdrawal or a transfer, this must always be an asset account. For deposits, this must be a revenue account. Can be used instead of the source_id. If the transaction is a deposit, the source_name can be filled in freely: the account will be created based on the name.
    #[serde(rename = "source_name", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "source_iban", skip_serializing_if = "Option::is_none")]
    pub source_iban: Option<String>,
    #[serde(rename = "source_type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<crate::models::AccountTypeProperty>,
    /// ID of the destination account. For a deposit or a transfer, this must always be an asset account. For withdrawals this must be an expense account.
    #[serde(rename = "destination_id")]
    pub destination_id: Option<String>,
    /// Name of the destination account. You can submit the name instead of the ID. For everything except transfers, the account will be auto-generated if unknown, so submitting a name is enough.
    #[serde(rename = "destination_name", skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    #[serde(rename = "destination_iban", skip_serializing_if = "Option::is_none")]
    pub destination_iban: Option<String>,
    #[serde(rename = "destination_type", skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<crate::models::AccountTypeProperty>,
    /// If the transaction has been reconciled already. When you set this, the amount can no longer be edited by the user.
    #[serde(rename = "reconciled", skip_serializing_if = "Option::is_none")]
    pub reconciled: Option<bool>,
    /// Optional. Use either this or the piggy_bank_name
    #[serde(rename = "piggy_bank_id", skip_serializing_if = "Option::is_none")]
    pub piggy_bank_id: Option<i32>,
    /// Optional. Use either this or the piggy_bank_id
    #[serde(rename = "piggy_bank_name", skip_serializing_if = "Option::is_none")]
    pub piggy_bank_name: Option<String>,
    /// Optional. Use either this or the bill_name
    #[serde(rename = "bill_id", skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<i32>,
    /// Optional. Use either this or the bill_id
    #[serde(rename = "bill_name", skip_serializing_if = "Option::is_none")]
    pub bill_name: Option<String>,
    /// Array of tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Reference to internal reference of other systems.
    #[serde(rename = "internal_reference", skip_serializing_if = "Option::is_none")]
    pub internal_reference: Option<String>,
    /// Reference to external ID in other systems.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// System generated identifier for original creator of transaction.
    #[serde(rename = "original_source", skip_serializing_if = "Option::is_none")]
    pub original_source: Option<String>,
    /// Reference to recurrence that made the transaction.
    #[serde(rename = "recurrence_id", skip_serializing_if = "Option::is_none")]
    pub recurrence_id: Option<i32>,
    /// Total number of transactions expected to be created by this recurrence repetition. Will be 0 if infinite.
    #[serde(rename = "recurrence_total", skip_serializing_if = "Option::is_none")]
    pub recurrence_total: Option<i32>,
    /// The # of the current transaction created under this recurrence.
    #[serde(rename = "recurrence_count", skip_serializing_if = "Option::is_none")]
    pub recurrence_count: Option<i32>,
    /// Internal ID of bunq transaction.
    #[serde(rename = "bunq_payment_id", skip_serializing_if = "Option::is_none")]
    pub bunq_payment_id: Option<String>,
    /// Hash value of original import transaction (for duplicate detection).
    #[serde(rename = "import_hash_v2", skip_serializing_if = "Option::is_none")]
    pub import_hash_v2: Option<String>,
    /// SEPA Clearing Code
    #[serde(rename = "sepa_cc", skip_serializing_if = "Option::is_none")]
    pub sepa_cc: Option<String>,
    /// SEPA Opposing Account Identifier
    #[serde(rename = "sepa_ct_op", skip_serializing_if = "Option::is_none")]
    pub sepa_ct_op: Option<String>,
    /// SEPA end-to-end Identifier
    #[serde(rename = "sepa_ct_id", skip_serializing_if = "Option::is_none")]
    pub sepa_ct_id: Option<String>,
    /// SEPA mandate identifier
    #[serde(rename = "sepa_db", skip_serializing_if = "Option::is_none")]
    pub sepa_db: Option<String>,
    /// SEPA Country
    #[serde(rename = "sepa_country", skip_serializing_if = "Option::is_none")]
    pub sepa_country: Option<String>,
    /// SEPA External Purpose indicator
    #[serde(rename = "sepa_ep", skip_serializing_if = "Option::is_none")]
    pub sepa_ep: Option<String>,
    /// SEPA Creditor Identifier
    #[serde(rename = "sepa_ci", skip_serializing_if = "Option::is_none")]
    pub sepa_ci: Option<String>,
    /// SEPA Batch ID
    #[serde(rename = "sepa_batch_id", skip_serializing_if = "Option::is_none")]
    pub sepa_batch_id: Option<String>,
    #[serde(rename = "interest_date", skip_serializing_if = "Option::is_none")]
    pub interest_date: Option<String>,
    #[serde(rename = "book_date", skip_serializing_if = "Option::is_none")]
    pub book_date: Option<String>,
    #[serde(rename = "process_date", skip_serializing_if = "Option::is_none")]
    pub process_date: Option<String>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "payment_date", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "invoice_date", skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
}

impl TransactionSplit {
    pub fn new(_type: Type, date: String, amount: String, description: String, source_id: Option<i32>, destination_id: Option<i32>) -> TransactionSplit {
        TransactionSplit {
            user: None,
            transaction_journal_id: None,
            _type,
            date,
            amount,
            description,
            order: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_name: None,
            currency_decimal_places: None,
            foreign_amount: None,
            foreign_currency_id: None,
            foreign_currency_code: None,
            foreign_currency_symbol: None,
            foreign_currency_decimal_places: None,
            budget_id: None,
            budget_name: None,
            category_id: None,
            category_name: None,
            source_id,
            source_name: None,
            source_iban: None,
            source_type: None,
            destination_id,
            destination_name: None,
            destination_iban: None,
            destination_type: None,
            reconciled: None,
            piggy_bank_id: None,
            piggy_bank_name: None,
            bill_id: None,
            bill_name: None,
            tags: None,
            notes: None,
            internal_reference: None,
            external_id: None,
            original_source: None,
            recurrence_id: None,
            recurrence_total: None,
            recurrence_count: None,
            bunq_payment_id: None,
            import_hash_v2: None,
            sepa_cc: None,
            sepa_ct_op: None,
            sepa_ct_id: None,
            sepa_db: None,
            sepa_country: None,
            sepa_ep: None,
            sepa_ci: None,
            sepa_batch_id: None,
            interest_date: None,
            book_date: None,
            process_date: None,
            due_date: None,
            payment_date: None,
            invoice_date: None,
        }
    }
}

/// Type of transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "reconciliation")]
    Reconciliation,
    #[serde(rename = "opening balance")]
    OpeningBalance,
}

