# SearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | **String** |  | [default to ]
**query** | Option<[**serde_json::Value**](.md)> |  | [optional]
**fulltext_filter** | Option<[**serde_json::Value**](.md)> |  | [optional]
**attr_filter** | Option<[**serde_json::Value**](.md)> |  | [optional]
**limit** | Option<**i32**> |  | [optional]
**offset** | Option<**i32**> |  | [optional]
**max_matches** | Option<**i32**> |  | [optional]
**sort** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**aggs** | Option<[**std::collections::HashMap<String, models::Aggregation>**](aggregation.md)> |  | [optional]
**expressions** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**highlight** | Option<[**models::Highlight**](highlight.md)> |  | [optional]
**source** | Option<[**serde_json::Value**](.md)> |  | [optional]
**options** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**profile** | Option<**bool**> |  | [optional]
**track_scores** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


