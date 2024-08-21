# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**start_date** | **String** |  | 
**phases** | [**Vec<models::EventPhase>**](EventPhase.md) |  | 
**partition_size** | Option<**i32**> |  | [optional]
**group_rewards** | Option<[**models::EventGroupRewards**](EventGroupRewards.md)> |  | [optional]
**cohort_settings** | Option<[**models::LeaderboardCohortSettings**](LeaderboardCohortSettings.md)> |  | [optional]
**permissions** | Option<[**models::ClientPermission**](ClientPermission.md)> |  | [optional]
**stores** | Option<**Vec<String>**> |  | [optional]
**symbol** | **String** |  | 
**score_rewards** | Option<[**Vec<models::EventRewardContent>**](EventRewardContent.md)> |  | [optional]
**schedule** | Option<[**models::Schedule**](Schedule.md)> |  | [optional]
**rank_rewards** | Option<[**Vec<models::EventRewardContent>**](EventRewardContent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


