# \DefaultApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**basic_accounts_admin_admin_user_post**](DefaultApi.md#basic_accounts_admin_admin_user_post) | **POST** /basic/accounts/admin/admin-user | 
[**basic_accounts_admin_admin_users_get**](DefaultApi.md#basic_accounts_admin_admin_users_get) | **GET** /basic/accounts/admin/admin-users | 
[**basic_accounts_admin_me_get**](DefaultApi.md#basic_accounts_admin_me_get) | **GET** /basic/accounts/admin/me | 
[**basic_accounts_available_device_id_get**](DefaultApi.md#basic_accounts_available_device_id_get) | **GET** /basic/accounts/available/device-id | 
[**basic_accounts_available_external_identity_get**](DefaultApi.md#basic_accounts_available_external_identity_get) | **GET** /basic/accounts/available/external_identity | 
[**basic_accounts_available_get**](DefaultApi.md#basic_accounts_available_get) | **GET** /basic/accounts/available | 
[**basic_accounts_available_third_party_get**](DefaultApi.md#basic_accounts_available_third_party_get) | **GET** /basic/accounts/available/third-party | 
[**basic_accounts_email_update_confirm_post**](DefaultApi.md#basic_accounts_email_update_confirm_post) | **POST** /basic/accounts/email-update/confirm | 
[**basic_accounts_email_update_init_post**](DefaultApi.md#basic_accounts_email_update_init_post) | **POST** /basic/accounts/email-update/init | 
[**basic_accounts_external_identity_delete**](DefaultApi.md#basic_accounts_external_identity_delete) | **DELETE** /basic/accounts/external_identity | 
[**basic_accounts_external_identity_post**](DefaultApi.md#basic_accounts_external_identity_post) | **POST** /basic/accounts/external_identity | 
[**basic_accounts_find_get**](DefaultApi.md#basic_accounts_find_get) | **GET** /basic/accounts/find | 
[**basic_accounts_get_personally_identifiable_information_get**](DefaultApi.md#basic_accounts_get_personally_identifiable_information_get) | **GET** /basic/accounts/get-personally-identifiable-information | 
[**basic_accounts_me_device_delete**](DefaultApi.md#basic_accounts_me_device_delete) | **DELETE** /basic/accounts/me/device | 
[**basic_accounts_me_get**](DefaultApi.md#basic_accounts_me_get) | **GET** /basic/accounts/me | 
[**basic_accounts_me_put**](DefaultApi.md#basic_accounts_me_put) | **PUT** /basic/accounts/me | 
[**basic_accounts_me_third_party_delete**](DefaultApi.md#basic_accounts_me_third_party_delete) | **DELETE** /basic/accounts/me/third-party | 
[**basic_accounts_password_update_confirm_post**](DefaultApi.md#basic_accounts_password_update_confirm_post) | **POST** /basic/accounts/password-update/confirm | 
[**basic_accounts_password_update_init_post**](DefaultApi.md#basic_accounts_password_update_init_post) | **POST** /basic/accounts/password-update/init | 
[**basic_accounts_register_post**](DefaultApi.md#basic_accounts_register_post) | **POST** /basic/accounts/register | 
[**basic_accounts_search_get**](DefaultApi.md#basic_accounts_search_get) | **GET** /basic/accounts/search | 



## basic_accounts_admin_admin_user_post

> crate::models::AccountPortalView basic_accounts_admin_admin_user_post(add_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_account_request** | Option<[**AddAccountRequest**](AddAccountRequest.md)> |  |  |

### Return type

[**crate::models::AccountPortalView**](AccountPortalView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_admin_admin_users_get

> crate::models::GetAdminsResponse basic_accounts_admin_admin_users_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAdminsResponse**](GetAdminsResponse.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_admin_me_get

> crate::models::AccountPortalView basic_accounts_admin_me_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountPortalView**](AccountPortalView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_device_id_get

> crate::models::AccountAvailableResponse basic_accounts_available_device_id_get(device_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** |  | [required] |

### Return type

[**crate::models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_external_identity_get

> crate::models::AccountAvailableResponse basic_accounts_available_external_identity_get(provider_service, user_id, provider_namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_service** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**provider_namespace** | Option<**String**> |  |  |

### Return type

[**crate::models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_get

> crate::models::AccountAvailableResponse basic_accounts_available_get(email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**crate::models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_available_third_party_get

> crate::models::AccountAvailableResponse basic_accounts_available_third_party_get(third_party, token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**third_party** | **String** |  | [required] |
**token** | **String** |  | [required] |

### Return type

[**crate::models::AccountAvailableResponse**](AccountAvailableResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_email_update_confirm_post

> crate::models::EmptyResponse basic_accounts_email_update_confirm_post(email_update_confirmation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_update_confirmation** | Option<[**EmailUpdateConfirmation**](EmailUpdateConfirmation.md)> |  |  |

### Return type

[**crate::models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_email_update_init_post

> crate::models::EmptyResponse basic_accounts_email_update_init_post(email_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_update_request** | Option<[**EmailUpdateRequest**](EmailUpdateRequest.md)> |  |  |

### Return type

[**crate::models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_external_identity_delete

> crate::models::CommonResponse basic_accounts_external_identity_delete(delete_external_identity_api_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_external_identity_api_request** | Option<[**DeleteExternalIdentityApiRequest**](DeleteExternalIdentityApiRequest.md)> |  |  |

### Return type

[**crate::models::CommonResponse**](CommonResponse.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_external_identity_post

> crate::models::AttachExternalIdentityApiResponse basic_accounts_external_identity_post(attach_external_identity_api_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attach_external_identity_api_request** | Option<[**AttachExternalIdentityApiRequest**](AttachExternalIdentityApiRequest.md)> |  |  |

### Return type

[**crate::models::AttachExternalIdentityApiResponse**](AttachExternalIdentityApiResponse.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_find_get

> crate::models::Account basic_accounts_find_get(query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_get_personally_identifiable_information_get

> crate::models::AccountPersonallyIdentifiableInformationResponse basic_accounts_get_personally_identifiable_information_get(query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |

### Return type

[**crate::models::AccountPersonallyIdentifiableInformationResponse**](AccountPersonallyIdentifiableInformationResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_device_delete

> crate::models::AccountPlayerView basic_accounts_me_device_delete(delete_devices_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_devices_request** | Option<[**DeleteDevicesRequest**](DeleteDevicesRequest.md)> |  |  |

### Return type

[**crate::models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_get

> crate::models::AccountPlayerView basic_accounts_me_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_put

> crate::models::AccountPlayerView basic_accounts_me_put(account_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_update** | Option<[**AccountUpdate**](AccountUpdate.md)> |  |  |

### Return type

[**crate::models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_me_third_party_delete

> crate::models::AccountPlayerView basic_accounts_me_third_party_delete(third_party_available_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**third_party_available_request** | Option<[**ThirdPartyAvailableRequest**](ThirdPartyAvailableRequest.md)> |  |  |

### Return type

[**crate::models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_password_update_confirm_post

> crate::models::EmptyResponse basic_accounts_password_update_confirm_post(password_update_confirmation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_update_confirmation** | Option<[**PasswordUpdateConfirmation**](PasswordUpdateConfirmation.md)> |  |  |

### Return type

[**crate::models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_password_update_init_post

> crate::models::EmptyResponse basic_accounts_password_update_init_post(password_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_update_request** | Option<[**PasswordUpdateRequest**](PasswordUpdateRequest.md)> |  |  |

### Return type

[**crate::models::EmptyResponse**](EmptyResponse.md)

### Authorization

[scope](../README.md#scope)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_register_post

> crate::models::AccountPlayerView basic_accounts_register_post(account_registration)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_registration** | Option<[**AccountRegistration**](AccountRegistration.md)> |  |  |

### Return type

[**crate::models::AccountPlayerView**](AccountPlayerView.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## basic_accounts_search_get

> crate::models::AccountSearchResponse basic_accounts_search_get(query, page, pagesize)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**page** | **i32** |  | [required] |
**pagesize** | **i32** |  | [required] |

### Return type

[**crate::models::AccountSearchResponse**](AccountSearchResponse.md)

### Authorization

[scope](../README.md#scope), [user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

