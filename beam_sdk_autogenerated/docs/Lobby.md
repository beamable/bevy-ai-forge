# Lobby

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lobby_id** | Option<**String**> |  | [optional]
**match_type** | Option<[**models::MatchType**](MatchType.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**host** | Option<**String**> |  | [optional]
**players** | Option<[**Vec<models::LobbyPlayer>**](LobbyPlayer.md)> |  | [optional][readonly]
**passcode** | Option<**String**> |  | [optional]
**restriction** | Option<[**models::LobbyRestriction**](LobbyRestriction.md)> |  | [optional]
**max_players** | Option<**i32**> |  | [optional]
**data** | Option<**std::collections::HashMap<String, String>**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


