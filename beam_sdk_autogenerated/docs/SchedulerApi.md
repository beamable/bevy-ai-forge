# \SchedulerApi

All URIs are relative to *https://api.beamable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_internal_scheduler_job_execute_post**](SchedulerApi.md#api_internal_scheduler_job_execute_post) | **POST** /api/internal/scheduler/job/execute | Called by the Dispatcher lambda function to start a job execution at the appropriate time.
[**api_internal_scheduler_job_post**](SchedulerApi.md#api_internal_scheduler_job_post) | **POST** /api/internal/scheduler/job | 
[**api_scheduler_job_job_id_activity_get**](SchedulerApi.md#api_scheduler_job_job_id_activity_get) | **GET** /api/scheduler/job/{jobId}/activity | 
[**api_scheduler_job_job_id_cancel_put**](SchedulerApi.md#api_scheduler_job_job_id_cancel_put) | **PUT** /api/scheduler/job/{jobId}/cancel | 
[**api_scheduler_job_job_id_delete**](SchedulerApi.md#api_scheduler_job_job_id_delete) | **DELETE** /api/scheduler/job/{jobId} | 
[**api_scheduler_job_job_id_get**](SchedulerApi.md#api_scheduler_job_job_id_get) | **GET** /api/scheduler/job/{jobId} | 
[**api_scheduler_job_job_id_next_executions_get**](SchedulerApi.md#api_scheduler_job_job_id_next_executions_get) | **GET** /api/scheduler/job/{jobId}/next-executions | 
[**api_scheduler_job_post**](SchedulerApi.md#api_scheduler_job_post) | **POST** /api/scheduler/job | 
[**api_scheduler_jobs_get**](SchedulerApi.md#api_scheduler_jobs_get) | **GET** /api/scheduler/jobs | 



## api_internal_scheduler_job_execute_post

> models::JobExecutionResult api_internal_scheduler_job_execute_post(job_execution_event)
Called by the Dispatcher lambda function to start a job execution at the appropriate time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_execution_event** | Option<[**JobExecutionEvent**](JobExecutionEvent.md)> |  |  |

### Return type

[**models::JobExecutionResult**](JobExecutionResult.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_internal_scheduler_job_post

> models::JobDefinition api_internal_scheduler_job_post(job_definition_save_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_definition_save_request** | Option<[**JobDefinitionSaveRequest**](JobDefinitionSaveRequest.md)> |  |  |

### Return type

[**models::JobDefinition**](JobDefinition.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_activity_get

> Vec<models::JobActivity> api_scheduler_job_job_id_activity_get(job_id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 1000]

### Return type

[**Vec<models::JobActivity>**](JobActivity.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_cancel_put

> serde_json::Value api_scheduler_job_job_id_cancel_put(job_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_delete

> serde_json::Value api_scheduler_job_job_id_delete(job_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_get

> models::JobDefinition api_scheduler_job_job_id_get(job_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::JobDefinition**](JobDefinition.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_job_id_next_executions_get

> Vec<String> api_scheduler_job_job_id_next_executions_get(job_id, from, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**from** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 1000]

### Return type

**Vec<String>**

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_job_post

> models::JobDefinition api_scheduler_job_post(job_definition_save_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_definition_save_request** | Option<[**JobDefinitionSaveRequest**](JobDefinitionSaveRequest.md)> |  |  |

### Return type

[**models::JobDefinition**](JobDefinition.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_scheduler_jobs_get

> Vec<models::JobDefinition> api_scheduler_jobs_get(source, name, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 1000]

### Return type

[**Vec<models::JobDefinition>**](JobDefinition.md)

### Authorization

[user](../README.md#user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

