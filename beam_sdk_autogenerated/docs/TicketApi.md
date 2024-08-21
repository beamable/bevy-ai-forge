# \TicketApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_matchmaking_tickets_id_delete**](TicketApi.md#api_matchmaking_tickets_id_delete) | **DELETE** /api/matchmaking/tickets/{id} | Cancel a pending ticket. If no ticket with the id exists, this will  still return a 204.
[**api_matchmaking_tickets_id_get**](TicketApi.md#api_matchmaking_tickets_id_get) | **GET** /api/matchmaking/tickets/{id} | Fetch a ticket by ID.
[**api_matchmaking_tickets_post**](TicketApi.md#api_matchmaking_tickets_post) | **POST** /api/matchmaking/tickets | Create a ticket representing 1 or more players to be matched  with others.



## api_matchmaking_tickets_id_delete

> serde_json::Value api_matchmaking_tickets_id_delete(id)
Cancel a pending ticket. If no ticket with the id exists, this will  still return a 204.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_id_get

> models::Ticket api_matchmaking_tickets_id_get(id)
Fetch a ticket by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Ticket ID | [required] |

### Return type

[**models::Ticket**](Ticket.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_matchmaking_tickets_post

> models::TicketReservationResponse api_matchmaking_tickets_post(ticket_reservation_request)
Create a ticket representing 1 or more players to be matched  with others.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_reservation_request** | Option<[**TicketReservationRequest**](TicketReservationRequest.md)> |  |  |

### Return type

[**models::TicketReservationResponse**](TicketReservationResponse.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

