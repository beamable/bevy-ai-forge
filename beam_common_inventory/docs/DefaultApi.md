# \DefaultApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**object_inventory_object_id_get**](DefaultApi.md#object_inventory_object_id_get) | **GET** /object/inventory/{objectId}/ | 
[**object_inventory_object_id_multipliers_get**](DefaultApi.md#object_inventory_object_id_multipliers_get) | **GET** /object/inventory/{objectId}/multipliers | 
[**object_inventory_object_id_post**](DefaultApi.md#object_inventory_object_id_post) | **POST** /object/inventory/{objectId}/ | 
[**object_inventory_object_id_preview_put**](DefaultApi.md#object_inventory_object_id_preview_put) | **PUT** /object/inventory/{objectId}/preview | 
[**object_inventory_object_id_proxy_reload_put**](DefaultApi.md#object_inventory_object_id_proxy_reload_put) | **PUT** /object/inventory/{objectId}/proxy/reload | 
[**object_inventory_object_id_put**](DefaultApi.md#object_inventory_object_id_put) | **PUT** /object/inventory/{objectId}/ | 
[**object_inventory_object_id_transaction_delete**](DefaultApi.md#object_inventory_object_id_transaction_delete) | **DELETE** /object/inventory/{objectId}/transaction | 
[**object_inventory_object_id_transfer_put**](DefaultApi.md#object_inventory_object_id_transfer_put) | **PUT** /object/inventory/{objectId}/transfer | 



## object_inventory_object_id_get

> crate::models::InventoryView object_inventory_object_id_get(object_id, scope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**scope** | Option<**String**> |  |  |

### Return type

[**crate::models::InventoryView**](InventoryView.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_multipliers_get

> crate::models::MultipliersGetResponse object_inventory_object_id_multipliers_get(object_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |

### Return type

[**crate::models::MultipliersGetResponse**](MultipliersGetResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_post

> crate::models::InventoryView object_inventory_object_id_post(object_id, inventory_query_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**inventory_query_request** | Option<[**InventoryQueryRequest**](InventoryQueryRequest.md)> |  |  |

### Return type

[**crate::models::InventoryView**](InventoryView.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_preview_put

> crate::models::PreviewVipBonusResponse object_inventory_object_id_preview_put(object_id, inventory_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**inventory_update_request** | Option<[**InventoryUpdateRequest**](InventoryUpdateRequest.md)> |  |  |

### Return type

[**crate::models::PreviewVipBonusResponse**](PreviewVipBonusResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_proxy_reload_put

> crate::models::CommonResponse object_inventory_object_id_proxy_reload_put(object_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |

### Return type

[**crate::models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_put

> crate::models::CommonResponse object_inventory_object_id_put(object_id, inventory_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**inventory_update_request** | Option<[**InventoryUpdateRequest**](InventoryUpdateRequest.md)> |  |  |

### Return type

[**crate::models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_transaction_delete

> crate::models::CommonResponse object_inventory_object_id_transaction_delete(object_id, end_transaction_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**end_transaction_request** | Option<[**EndTransactionRequest**](EndTransactionRequest.md)> |  |  |

### Return type

[**crate::models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope), [developer](../README.md#developer), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## object_inventory_object_id_transfer_put

> crate::models::CommonResponse object_inventory_object_id_transfer_put(object_id, transfer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | Gamertag of the player.Underlying objectId type is integer in format int64. | [required] |
**transfer_request** | Option<[**TransferRequest**](TransferRequest.md)> |  |  |

### Return type

[**crate::models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope), [developer](../README.md#developer), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

