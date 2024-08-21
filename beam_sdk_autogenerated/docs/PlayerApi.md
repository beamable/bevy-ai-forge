# \PlayerApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_parties_invites_get**](PlayerApi.md#api_players_player_id_parties_invites_get) | **GET** /api/players/{playerId}/parties/invites | Return list of party invites for player.



## api_players_player_id_parties_invites_get

> models::PartyInvitesForPlayerResponse api_players_player_id_parties_invites_get(player_id)
Return list of party invites for player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | PlayerId | [required] |

### Return type

[**models::PartyInvitesForPlayerResponse**](PartyInvitesForPlayerResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

