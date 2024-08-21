# \PlayerPresenceApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_players_player_id_presence_get**](PlayerPresenceApi.md#api_players_player_id_presence_get) | **GET** /api/players/{playerId}/presence | 
[**api_players_player_id_presence_put**](PlayerPresenceApi.md#api_players_player_id_presence_put) | **PUT** /api/players/{playerId}/presence | 
[**api_players_player_id_presence_status_put**](PlayerPresenceApi.md#api_players_player_id_presence_status_put) | **PUT** /api/players/{playerId}/presence/status | 



## api_players_player_id_presence_get

> models::OnlineStatus api_players_player_id_presence_get(player_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |

### Return type

[**models::OnlineStatus**](OnlineStatus.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_presence_put

> serde_json::Value api_players_player_id_presence_put(player_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_players_player_id_presence_status_put

> models::OnlineStatus api_players_player_id_presence_status_put(player_id, set_presence_status_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** |  | [required] |
**set_presence_status_request** | Option<[**SetPresenceStatusRequest**](SetPresenceStatusRequest.md)> |  |  |

### Return type

[**models::OnlineStatus**](OnlineStatus.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

