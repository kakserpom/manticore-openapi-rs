# Highlight

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fieldnames** | Option<**Vec<String>**> |  | [optional]
**fields** | Option<[**Vec<models::HighlightField>**](highlightField.md)> |  | [optional]
**encoder** | Option<**String**> |  | [optional]
**highlight_query** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**pre_tags** | Option<**String**> |  | [optional][default to <strong>]
**post_tags** | Option<**String**> |  | [optional][default to </strong>]
**no_match_size** | Option<**i32**> |  | [optional]
**fragment_size** | Option<**i32**> |  | [optional][default to 256]
**number_of_fragments** | Option<**i32**> |  | [optional][default to 0]
**limit** | Option<**i32**> |  | [optional][default to 256]
**limit_words** | Option<**i32**> |  | [optional][default to 0]
**limit_snippets** | Option<**i32**> |  | [optional][default to 0]
**limits_per_field** | Option<**bool**> |  | [optional][default to false]
**use_boundaries** | Option<**bool**> |  | [optional][default to false]
**force_all_words** | Option<**bool**> |  | [optional][default to false]
**allow_empty** | Option<**bool**> |  | [optional][default to false]
**emit_zones** | Option<**bool**> |  | [optional][default to false]
**force_snippets** | Option<**bool**> |  | [optional][default to false]
**around** | Option<**i32**> |  | [optional][default to 5]
**start_snippet_id** | Option<**i32**> |  | [optional][default to 1]
**html_strip_mode** | Option<**String**> |  | [optional]
**snippet_boundary** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


