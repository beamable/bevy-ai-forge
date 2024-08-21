# EventPlayerStateView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**running** | **bool** |  | 
**all_phases** | [**Vec<models::EventPlayerPhaseView>**](EventPlayerPhaseView.md) |  | 
**rank** | **i64** |  | 
**score** | **f64** |  | 
**current_phase** | Option<[**models::EventPlayerPhaseView**](EventPlayerPhaseView.md)> |  | [optional]
**seconds_remaining** | **i64** |  | 
**id** | **String** |  | 
**leaderboard_id** | **String** |  | 
**rank_rewards** | [**Vec<models::EventRewardState>**](EventRewardState.md) |  | 
**group_rewards** | Option<[**models::EventPlayerGroupState**](EventPlayerGroupState.md)> |  | [optional]
**score_rewards** | [**Vec<models::EventRewardState>**](EventRewardState.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


