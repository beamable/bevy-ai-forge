# PaymentAuditEntryViewModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**providerid** | **String** |  | 
**history** | [**Vec<crate::models::PaymentHistoryEntryViewModel>**](PaymentHistoryEntryViewModel.md) |  | 
**txid** | **i64** |  | 
**providername** | **String** |  | 
**version** | Option<**String**> |  | [optional]
**obtain_items** | Option<[**Vec<crate::models::ItemCreateRequest>**](ItemCreateRequest.md)> |  | [optional]
**txstate** | **String** |  | 
**updated** | Option<**i64**> |  | [optional]
**obtain_currency** | Option<[**Vec<crate::models::CurrencyChange>**](CurrencyChange.md)> |  | [optional]
**entitlements** | [**Vec<crate::models::EntitlementGenerator>**](EntitlementGenerator.md) |  | 
**details** | [**crate::models::PaymentDetailsEntryViewModel**](PaymentDetailsEntryViewModel.md) |  | 
**replay_guard_value** | Option<**String**> |  | [optional]
**gt** | **i64** |  | 
**created** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


