# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**in_flight** | Option<[**Vec<models::InFlightMessage>**](InFlightMessage.md)> |  | [optional]
**name** | **String** |  | 
**enrollment_type** | **String** |  | 
**donations** | Option<[**Vec<models::DonationRequest>**](DonationRequest.md)> |  | [optional]
**free_slots** | **i32** |  | 
**maybe_donations** | Option<[**std::collections::HashMap<String, models::DonationRequest>**](DonationRequest.md)> |  | [optional]
**tag** | Option<**String**> |  | [optional]
**can_update_motd** | Option<**bool**> |  | [optional]
**shard** | Option<**String**> |  | [optional]
**can_update_slogan** | Option<**bool**> |  | [optional]
**leader** | **i64** |  | 
**slogan** | **String** |  | 
**requirement** | **i64** |  | 
**motd** | **String** |  | 
**version** | Option<**i32**> |  | [optional]
**id** | **i64** |  | 
**client_data** | Option<**String**> |  | [optional]
**roles** | Option<[**Vec<models::GroupRole>**](GroupRole.md)> |  | [optional]
**scores** | **std::collections::HashMap<String, String>** |  | 
**can_update_enrollment** | Option<**bool**> |  | [optional]
**members** | [**Vec<models::Member>**](Member.md) |  | 
**can_disband** | Option<**bool**> |  | [optional]
**r#type** | [**models::GroupType**](GroupType.md) |  | 
**max_size** | **i32** |  | 
**sub_groups** | [**Vec<models::Group>**](Group.md) |  | 
**created** | **i64** | Milliseconds since midnight, January 1, 1970 UTC | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


