# \PresenceApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_presence_query_post**](PresenceApi.md#api_presence_query_post) | **POST** /api/presence/query | 



## api_presence_query_post

> models::PlayersStatusResponse api_presence_query_post(online_status_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**online_status_query** | Option<[**OnlineStatusQuery**](OnlineStatusQuery.md)> |  |  |

### Return type

[**models::PlayersStatusResponse**](PlayersStatusResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

