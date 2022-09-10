# \AuthorDataApi

All URIs are relative to */graph/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_graph_get_author**](AuthorDataApi.md#get_graph_get_author) | **GET** /author/{author_id} | Details about an author
[**get_graph_get_author_papers**](AuthorDataApi.md#get_graph_get_author_papers) | **GET** /author/{author_id}/papers | Details about an author's papers
[**get_graph_get_author_search**](AuthorDataApi.md#get_graph_get_author_search) | **GET** /author/search | Search for authors by name



## get_graph_get_author

> crate::models::AuthorWithPapers get_graph_get_author(author_id, fields)
Details about an author

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | **String** |  | [required] |
**fields** | Option<**String**> | A comma-separated list of the fields to be returned.<br><br> The following case-sensitive author fields are recognized: <ul>     <li><code>authorId</code> - Always included</li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>name</code> - Included if no fields are specified</li>     <li><code>aliases</code></li>     <li><code>affiliations</code></li>     <li><code>homepage</code></li>     <li><code>paperCount</code></li>         <li><code>citationCount</code></li>         <li><code>hIndex</code></li>             <li><code>papers</code></li>         <ul>             <li><code>paperId</code> - Always included</li>             <li><code>externalIds</code></li>             <li><code>url</code></li>             <li><code>title</code> - Included if no fields are specified</li>             <li><code>abstract</code></li>             <li><code>venue</code></li>             <li><code>year</code></li>             <li><code>referenceCount</code></li>             <li><code>citationCount</code></li>             <li><code>influentialCitationCount</code></li>             <li><code>isOpenAccess</code></li>             <li><code>fieldsOfStudy</code></li>             <li><code>s2FieldsOfStudy</code></li>             <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>             <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>             <li><code>journal</code> - Journal name, volume, and pages, if available</li>             <li><code>authors</code> - Up to 500 will be returned</li>                 <ul>                     <li><code>authorId</code> - Always included</li>                     <li><code>name</code> - Always included</li>                     <li>To get more detailed information about an author's papers, use the <code>/author/{author_id}/papers</code> endpoint</li>                 </ul>         </ul> </ul> <br><br> Examples: <ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/1741101</code></li>     <ul>         <li>Returns the author's always included field of authorId plus the name.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/1741101?fields=aliases,papers</code></li>     <ul>         <li>Returns the author's authorId, aliases, and list of papers.  </li>         <li>Each paper has its paperId plus its title.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/1741101?fields=url,papers.abstract,papers.authors</code></li>     <ul>         <li>Returns the author's authorId, url, and list of papers.  </li>         <li>Each paper has its paperId, abstract, and list of authors.</li>         <li>In that list of authors, each author has their authorId and name.</li>     </ul> </ul> |  |

### Return type

[**crate::models::AuthorWithPapers**](AuthorWithPapers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_graph_get_author_papers

> crate::models::PaperBatch get_graph_get_author_papers(author_id, offset, limit, fields)
Details about an author's papers

Fetch the papers of an author in batches.<br><br> Only retrieves the most recent 10,000 citations/references for papers belonging to the batch To retrieve the full set of citations for a paper, use the /paper/{paper_id}/citations endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | **String** |  | [required] |
**offset** | Option<**i32**> | When returning a list of results, start with the element at this position in the list.<br> The sum of <code>offset</code> and <code>limit</code> must be < 10000 |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return.<br> Must be <= 1000 |  |[default to 100]
**fields** | Option<**String**> | A comma-separated list of the fields to be returned.<br><br>  The following case-sensitive paper fields are recognized: <ul>     <li><code>paperId</code> - Always included</li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>title</code> - Included if no fields are specified</li>     <li><code>abstract</code></li>     <li><code>venue</code></li>     <li><code>year</code></li>     <li><code>referenceCount</code></li>     <li><code>citationCount</code></li>     <li><code>influentialCitationCount</code></li>     <li><code>isOpenAccess</code></li>     <li><code>fieldsOfStudy</code></li>     <li><code>s2FieldsOfStudy</code></li>     <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>     <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>     <li><code>journal</code> - Journal name, volume, and pages, if available</li>     <li><code>authors</code> - Up to 500 will be returned.  Will include: <code>authorId</code> & <code>name</code></li>     <li><code>citations</code></li>     <ul>         <li>Same fields supported as for papers above</li>         <li>Total number of citations will be truncated at 10,000 for the entire batch.</li>         <li>To fetch more citations per paper, reduce the number of papers in the batch with <code>limit=</code> or use the <code>/paper/{paper_id}/citations</code> endpoint.</li>     </ul>     <li><code>references</code></li>     <ul>         <li>Same fields supported as for papers above</li>         <li>Total number of references will be truncated at 10,000 for the entire batch.</li>         <li>To fetch more references per paper, reduce the number of papers in the batch with <code>limit=</code> or use the <code>/paper/{paper_id}/references</code> endpoint.</li>     </ul> </ul> <br><br> Examples: <ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/1741101/papers</code></li>     <ul>         <li>Return with offset=0, and data is a list of the first 100 papers.</li>         <li>Each paper has its paperId and title.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/1741101/papers?fields=url,year,authors&limit=2</code></li>     <ul>         <li>Returns with offset=0, next=2, and data is a list of 2 papers.</li>         <li>Each paper has its paperId, url, year, and list of authors.</li>         <li>Each author has their authorId and name.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/1741101/papers?fields=citations.authors&offset=260</code></li>     <ul>         <li>Returns with offset=260, and data is a list of the last 4 papers.</li>         <li>Each paper has its paperId and a list of citations.</li>         <li>Each citation has its paperId and a list of authors.</li>         <li>Each author has their authorId and name.</li>     </ul> </ul>   |  |

### Return type

[**crate::models::PaperBatch**](PaperBatch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_graph_get_author_search

> crate::models::AuthorSearchBatch get_graph_get_author_search(query, offset, limit, fields)
Search for authors by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | A plain-text search query string. No special query syntax is supported.  Specifying <code>papers</code> fields in the request will return all papers linked to each author in the results, set a <code>limit</code> on the search results to reduce output size and latency. | [required] |
**offset** | Option<**i32**> | When returning a list of results, start with the element at this position in the list.<br> The sum of <code>offset</code> and <code>limit</code> must be < 10000 |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return.<br> Must be <= 1000 |  |[default to 100]
**fields** | Option<**String**> | A comma-separated list of the fields to be returned.<br><br>  The following case-sensitive author fields are recognized: <ul>     <li><code>authorId</code> - Always included</li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>name</code> - Included if no fields are specified</li>     <li><code>aliases</code></li>     <li><code>affiliations</code></li>     <li><code>homepage</code></li>     <li><code>paperCount</code></li>     <li><code>citationCount</code></li>     <li><code>hIndex</code></li>     <li><code>papers</code></li>     <ul>         <li><code>paperId</code> - Always included</li>         <li><code>externalIds</code></li>         <li><code>url</code></li>         <li><code>title</code> - Included if no fields are specified</li>         <li><code>abstract</code></li>         <li><code>venue</code></li>         <li><code>year</code></li>         <li><code>referenceCount</code></li>         <li><code>citationCount</code></li>         <li><code>influentialCitationCount</code></li>         <li><code>isOpenAccess</code></li>         <li><code>fieldsOfStudy</code></li>         <li><code>s2FieldsOfStudy</code></li>         <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>         <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>         <li><code>journal</code> - Journal name, volume, and pages, if available</li>         <li><code>authors</code> - Up to 500 will be returned</li>             <ul>                 <li><code>authorId</code> - Always included</li>                 <li><code>name</code> - Always included</li>                 <li>To get more detailed information about an author's papers, use the <code>/author/{author_id}/papers</code> endpoint</li>             </ul>         </li>     </ul>     </ul> <br><br> Examples: <ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/search?query=adam+smith</code></li>     <ul>         <li>Returns with total=490, offset=0, next=100, and data is a list of 100 authors.</li>         <li>Each author has the always included fields of authorId plus the name.  </li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/search?query=adam+smith&fields=name,aliases,url,papers.title,papers.year&limit=5</code></li>     <ul>         <li>Returns with total=490, offset=0, next=5, and data is a list of 5 authors.</li>         <li>Each author has authorId, name, aliases, url, and a list of their papers title and year.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/author/search?query=totalGarbageNonsense</code></li>     <ul>         <li>Returns with total = 0, offset=0, and data is a list of 0 author.</li>     </ul> </ul> |  |

### Return type

[**crate::models::AuthorSearchBatch**](AuthorSearchBatch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

