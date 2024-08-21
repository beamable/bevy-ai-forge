# \PartyApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_parties_id_get**](PartyApi.md#api_parties_id_get) | **GET** /api/parties/{id} | Return the status of a party.
[**api_parties_id_invite_delete**](PartyApi.md#api_parties_id_invite_delete) | **DELETE** /api/parties/{id}/invite | Cancel party invitation.
[**api_parties_id_invite_post**](PartyApi.md#api_parties_id_invite_post) | **POST** /api/parties/{id}/invite | Invite a player to a party
[**api_parties_id_members_delete**](PartyApi.md#api_parties_id_members_delete) | **DELETE** /api/parties/{id}/members | Remove the requested player from the party. The leader is able to remove anyone. Others may  only remove themselves without error.
[**api_parties_id_metadata_put**](PartyApi.md#api_parties_id_metadata_put) | **PUT** /api/parties/{id}/metadata | Updates party state.
[**api_parties_id_promote_put**](PartyApi.md#api_parties_id_promote_put) | **PUT** /api/parties/{id}/promote | Promote a party member to leader.
[**api_parties_id_put**](PartyApi.md#api_parties_id_put) | **PUT** /api/parties/{id} | Join a party
[**api_parties_post**](PartyApi.md#api_parties_post) | **POST** /api/parties | Create a party for the current player.



## api_parties_id_get

> models::Party api_parties_id_get(id)
Return the status of a party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |

### Return type

[**models::Party**](Party.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_invite_delete

> serde_json::Value api_parties_id_invite_delete(id, cancel_invite_to_party)
Cancel party invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**cancel_invite_to_party** | Option<[**CancelInviteToParty**](CancelInviteToParty.md)> | Player to be uninvited |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_invite_post

> serde_json::Value api_parties_id_invite_post(id, invite_to_party)
Invite a player to a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**invite_to_party** | Option<[**InviteToParty**](InviteToParty.md)> | Player to invite to the party |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_members_delete

> serde_json::Value api_parties_id_members_delete(id, leave_party)
Remove the requested player from the party. The leader is able to remove anyone. Others may  only remove themselves without error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**leave_party** | Option<[**LeaveParty**](LeaveParty.md)> | The leave party request |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_metadata_put

> models::Party api_parties_id_metadata_put(id, update_party)
Updates party state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**update_party** | Option<[**UpdateParty**](UpdateParty.md)> | Argument to pass to the party actor to update state. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_promote_put

> models::Party api_parties_id_promote_put(id, promote_new_leader)
Promote a party member to leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |
**promote_new_leader** | Option<[**PromoteNewLeader**](PromoteNewLeader.md)> | Player to promote to leader |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_id_put

> models::Party api_parties_id_put(id)
Join a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the party | [required] |

### Return type

[**models::Party**](Party.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_parties_post

> models::Party api_parties_post(create_party)
Create a party for the current player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_party** | Option<[**CreateParty**](CreateParty.md)> | Argument to pass to the party actor to initialize state. |  |

### Return type

[**models::Party**](Party.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

