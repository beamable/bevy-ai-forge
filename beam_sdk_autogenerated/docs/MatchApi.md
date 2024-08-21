# \MatchApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_matchmaking_matches_id_get**](MatchApi.md#api_matchmaking_matches_id_get) | **GET** /api/matchmaking/matches/{id} | Fetch a match by ID.



## api_matchmaking_matches_id_get

> models::Match api_matchmaking_matches_id_get(id)
Fetch a match by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Match ID | [required] |

### Return type

[**models::Match**](Match.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

