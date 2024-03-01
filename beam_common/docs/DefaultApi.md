# \DefaultApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_auth_token_get**](DefaultApi.md#basic_auth_token_get) | **GET** /basic/auth/token | 
[**basic_auth_token_list_get**](DefaultApi.md#basic_auth_token_list_get) | **GET** /basic/auth/token/list | 
[**basic_auth_token_post**](DefaultApi.md#basic_auth_token_post) | **POST** /basic/auth/token | 
[**basic_auth_token_revoke_put**](DefaultApi.md#basic_auth_token_revoke_put) | **PUT** /basic/auth/token/revoke | 



## basic_auth_token_get

> crate::models::Token basic_auth_token_get(token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_list_get

> crate::models::ListTokenResponse basic_auth_token_list_get(page_size, page, gamer_tag_or_account_id, cid, pid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | **i32** |  | [required] |
**page** | **i32** |  | [required] |
**gamer_tag_or_account_id** | **i64** |  | [required] |
**cid** | Option<**i64**> |  |  |
**pid** | Option<**String**> |  |  |

### Return type

[**crate::models::ListTokenResponse**](ListTokenResponse.md)

### Authorization

[scope](../README.md#scope), [developer](../README.md#developer), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_post

> crate::models::TokenResponse basic_auth_token_post(token_request_wrapper)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request_wrapper** | Option<[**TokenRequestWrapper**](TokenRequestWrapper.md)> |  |  |

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_auth_token_revoke_put

> crate::models::CommonResponse basic_auth_token_revoke_put(revoke_token_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_token_request** | Option<[**RevokeTokenRequest**](RevokeTokenRequest.md)> |  |  |

### Return type

[**crate::models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope), [developer](../README.md#developer), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

