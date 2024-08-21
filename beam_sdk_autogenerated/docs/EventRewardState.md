# EventRewardState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pending_inventory_rewards** | [**models::EventInventoryPendingRewards**](EventInventoryPendingRewards.md) |  | 
**currencies** | Option<[**Vec<models::EventInventoryRewardCurrency>**](EventInventoryRewardCurrency.md)> |  | [optional]
**pending_currency_rewards** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**pending_item_rewards** | Option<[**Vec<models::ItemCreateRequest>**](ItemCreateRequest.md)> |  | [optional]
**items** | Option<[**Vec<models::EventInventoryRewardItem>**](EventInventoryRewardItem.md)> |  | [optional]
**min** | **f64** |  | 
**max** | Option<**f64**> |  | [optional]
**earned** | **bool** |  | 
**claimed** | **bool** |  | 
**pending_entitlement_rewards** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**obtain** | Option<[**Vec<models::EventRewardObtain>**](EventRewardObtain.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


