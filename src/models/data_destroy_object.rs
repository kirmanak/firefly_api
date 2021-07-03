/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataDestroyObject {
    #[serde(rename = "budgets")]
    Budgets,
    #[serde(rename = "bills")]
    Bills,
    #[serde(rename = "piggy_banks")]
    PiggyBanks,
    #[serde(rename = "rules")]
    Rules,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "categories")]
    Categories,
    #[serde(rename = "tags")]
    Tags,
    #[serde(rename = "object_groups")]
    ObjectGroups,
    #[serde(rename = "accounts")]
    Accounts,
    #[serde(rename = "asset_accounts")]
    AssetAccounts,
    #[serde(rename = "expense_accounts")]
    ExpenseAccounts,
    #[serde(rename = "revenue_accounts")]
    RevenueAccounts,
    #[serde(rename = "liabilities")]
    Liabilities,
    #[serde(rename = "transactions")]
    Transactions,
    #[serde(rename = "withdrawals")]
    Withdrawals,
    #[serde(rename = "deposits")]
    Deposits,
    #[serde(rename = "transfers")]
    Transfers,

}

impl ToString for DataDestroyObject {
    fn to_string(&self) -> String {
        match self {
            Self::Budgets => String::from("budgets"),
            Self::Bills => String::from("bills"),
            Self::PiggyBanks => String::from("piggy_banks"),
            Self::Rules => String::from("rules"),
            Self::Recurring => String::from("recurring"),
            Self::Categories => String::from("categories"),
            Self::Tags => String::from("tags"),
            Self::ObjectGroups => String::from("object_groups"),
            Self::Accounts => String::from("accounts"),
            Self::AssetAccounts => String::from("asset_accounts"),
            Self::ExpenseAccounts => String::from("expense_accounts"),
            Self::RevenueAccounts => String::from("revenue_accounts"),
            Self::Liabilities => String::from("liabilities"),
            Self::Transactions => String::from("transactions"),
            Self::Withdrawals => String::from("withdrawals"),
            Self::Deposits => String::from("deposits"),
            Self::Transfers => String::from("transfers"),
        }
    }
}



