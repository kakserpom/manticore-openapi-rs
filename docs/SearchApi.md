# \SearchApi

All URIs are relative to *http://127.0.0.1:9308*

Method | HTTP request | Description
------------- | ------------- | -------------
[**percolate**](SearchApi.md#percolate) | **POST** /pq/{index}/search | Perform reverse search on a percolate index
[**search**](SearchApi.md#search) | **POST** /search | Performs a search on an index



## percolate

> models::SearchResponse percolate(index, percolate_request)
Perform reverse search on a percolate index

Performs a percolate search.  This method must be used only on percolate indexes.  Expects two parameters: the index name and an object with array of documents to be tested. An example of the documents object:    ```   {     \"query\":     {       \"percolate\":       {         \"document\":         {           \"content\":\"sample content\"         }       }     }   }   ```  Responds with an object with matched stored queries:     ```   {     'timed_out':false,     'hits':     {       'total':2,       'max_score':1,       'hits':       [         {           '_index':'idx_pq_1',           '_type':'doc',           '_id':'2',           '_score':'1',           '_source':           {             'query':             {               'match':{'title':'some'}             }           }         },         {           '_index':'idx_pq_1',           '_type':'doc',           '_id':'5',           '_score':'1',           '_source':           {             'query':             {               'ql':'some | none'             }           }         }       ]     }   }   ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Name of the percolate index | [required] |
**percolate_request** | [**PercolateRequest**](PercolateRequest.md) |  | [required] |

### Return type

[**models::SearchResponse**](searchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> models::SearchResponse search(search_request)
Performs a search on an index

 The method expects an object with the following mandatory properties: * the name of the index to search * the match query object For details, see the documentation on [**SearchRequest**](SearchRequest.md) The method returns an object with the following properties: - took: the time taken to execute the search query. - timed_out: a boolean indicating whether the query timed out. - hits: an object with the following properties:    - total: the total number of hits found.    - hits: an array of hit objects, where each hit object represents a matched document. Each hit object has the following properties:      - _id: the ID of the matched document.      - _score: the score of the matched document.      - _source: the source data of the matched document.  In addition, if profiling is enabled, the response will include an additional array with profiling information attached. Here is an example search response:    ```   {     'took':10,     'timed_out':false,     'hits':     {       'total':2,       'hits':       [         {'_id':'1','_score':1,'_source':{'gid':11}},         {'_id':'2','_score':1,'_source':{'gid':12}}       ]     }   }   ```  For more information about the match query syntax and additional parameters that can be added to request and response, please see the documentation [here](https://manual.manticoresearch.com/Searching/Full_text_matching/Basic_usage#HTTP-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_request** | [**SearchRequest**](SearchRequest.md) |  | [required] |

### Return type

[**models::SearchResponse**](searchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

