# \OpenAiApi

All URIs are relative to *https://api.beamable.com/basic/1714908866696208.DE_1714908866696209.micro_ForgeService*

Method | HTTP request | Description
------------- | ------------- | -------------
[**open_ai_authenticate_post**](OpenAiApi.md#open_ai_authenticate_post) | **POST** /OpenAI/authenticate | 
[**open_ai_inventory_put_post**](OpenAiApi.md#open_ai_inventory_put_post) | **POST** /OpenAI/inventory/put | 
[**open_ai_inventory_state_post**](OpenAiApi.md#open_ai_inventory_state_post) | **POST** /OpenAI/inventory/state | 



## open_ai_authenticate_post

> crate::models::BeamablePeriodCommonPeriodFederatedAuthenticationResponse open_ai_authenticate_post(open_ai_authenticate_request_args)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_ai_authenticate_request_args** | Option<[**OpenAiAuthenticateRequestArgs**](OpenAiAuthenticateRequestArgs.md)> |  |  |

### Return type

[**crate::models::BeamablePeriodCommonPeriodFederatedAuthenticationResponse**](Beamable.Common.FederatedAuthenticationResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_ai_inventory_put_post

> crate::models::BeamablePeriodCommonPeriodFederatedInventoryProxyState open_ai_inventory_put_post(open_ai_inventory_put_request_args)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_ai_inventory_put_request_args** | Option<[**OpenAiInventoryPutRequestArgs**](OpenAiInventoryPutRequestArgs.md)> |  |  |

### Return type

[**crate::models::BeamablePeriodCommonPeriodFederatedInventoryProxyState**](Beamable.Common.FederatedInventoryProxyState.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_ai_inventory_state_post

> crate::models::BeamablePeriodCommonPeriodFederatedInventoryProxyState open_ai_inventory_state_post(open_ai_inventory_state_request_args)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_ai_inventory_state_request_args** | Option<[**OpenAiInventoryStateRequestArgs**](OpenAiInventoryStateRequestArgs.md)> |  |  |

### Return type

[**crate::models::BeamablePeriodCommonPeriodFederatedInventoryProxyState**](Beamable.Common.FederatedInventoryProxyState.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

