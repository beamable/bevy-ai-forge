# \AuthApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_auth_refresh_token_post**](AuthApi.md#api_auth_refresh_token_post) | **POST** /api/auth/refresh-token | Generate a new access token for previously authenticated account.
[**api_auth_server_post**](AuthApi.md#api_auth_server_post) | **POST** /api/auth/server | Generate a new access token for a machine with a shared secret



## api_auth_refresh_token_post

> models::AuthResponse api_auth_refresh_token_post(refresh_token_auth_request)
Generate a new access token for previously authenticated account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_auth_request** | Option<[**RefreshTokenAuthRequest**](RefreshTokenAuthRequest.md)> | `RefreshTokenAuthRequest` |  |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_auth_server_post

> models::ServerTokenResponse api_auth_server_post(server_token_auth_request)
Generate a new access token for a machine with a shared secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_token_auth_request** | Option<[**ServerTokenAuthRequest**](ServerTokenAuthRequest.md)> | `ServerTokenAuthRequest` |  |

### Return type

[**models::ServerTokenResponse**](ServerTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

