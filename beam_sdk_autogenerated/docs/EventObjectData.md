# EventObjectData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**in_flight** | Option<[**Vec<models::InFlightMessage>**](InFlightMessage.md)> |  | [optional]
**start_time** | Option<**i64**> |  | [optional]
**root_event_id** | Option<**String**> |  | [optional]
**origin_type** | Option<**String**> |  | [optional]
**state** | [**models::EventState**](EventState.md) |  | 
**running** | **bool** |  | 
**phase** | Option<[**models::EventPhaseRuntime**](EventPhaseRuntime.md)> |  | [optional]
**permissions** | Option<[**models::ClientPermission**](ClientPermission.md)> |  | [optional]
**last_child_event_id** | Option<**String**> |  | [optional]
**end_time** | Option<**i64**> |  | [optional]
**id** | **String** |  | 
**origin** | Option<**String**> |  | [optional]
**created_at** | Option<**i64**> |  | [optional]
**content** | [**models::Event**](Event.md) |  | 
**done** | **bool** |  | 
**leaderboard_id** | **String** |  | 
**phase_times** | Option<[**Vec<models::EventPhaseTime>**](EventPhaseTime.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


