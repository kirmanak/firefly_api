# Budget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**order** | Option<**i32**> |  | [optional][readonly]
**auto_budget_type** | Option<**String**> | The type of auto-budget that Firefly III must create. | [optional]
**auto_budget_currency_id** | Option<**i32**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**auto_budget_currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**auto_budget_amount** | Option<**String**> |  | [optional]
**auto_budget_period** | Option<**String**> | Period for the auto budget | [optional]
**spent** | Option<[**Vec<crate::models::BudgetSpent>**](BudgetSpent.md)> | Information on how much was spent in this budget. Is only filled in when the start and end date are submitted. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


