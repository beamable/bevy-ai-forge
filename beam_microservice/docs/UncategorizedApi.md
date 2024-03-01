# \UncategorizedApi

All URIs are relative to *https://api.beamable.com/basic/1714908866696208.DE_1714908866696209.micro_ForgeService*

Method | HTTP request | Description
------------- | ------------- | -------------
[**say_hi_post**](UncategorizedApi.md#say_hi_post) | **POST** /SayHi | 
[**sell_sword_post**](UncategorizedApi.md#sell_sword_post) | **POST** /SellSword | 
[**start_forging_sword_post**](UncategorizedApi.md#start_forging_sword_post) | **POST** /StartForgingSword | 



## say_hi_post

> String say_hi_post(say_hi_request_args)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**say_hi_request_args** | Option<[**SayHiRequestArgs**](SayHiRequestArgs.md)> |  |  |

### Return type

**String**

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell_sword_post

> bool sell_sword_post(sell_sword_request_args)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sell_sword_request_args** | Option<[**SellSwordRequestArgs**](SellSwordRequestArgs.md)> |  |  |

### Return type

**bool**

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_forging_sword_post

> bool start_forging_sword_post()


### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

