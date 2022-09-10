# \PaperDataApi

All URIs are relative to */graph/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_graph_get_paper**](PaperDataApi.md#get_graph_get_paper) | **GET** /paper/{paper_id} | Details about a paper
[**get_graph_get_paper_authors**](PaperDataApi.md#get_graph_get_paper_authors) | **GET** /paper/{paper_id}/authors | Details about a paper's authors
[**get_graph_get_paper_citations**](PaperDataApi.md#get_graph_get_paper_citations) | **GET** /paper/{paper_id}/citations | Details about a paper's citations
[**get_graph_get_paper_references**](PaperDataApi.md#get_graph_get_paper_references) | **GET** /paper/{paper_id}/references | Details about a paper's references
[**get_graph_get_paper_search**](PaperDataApi.md#get_graph_get_paper_search) | **GET** /paper/search | Search for papers by keyword



## get_graph_get_paper

> crate::models::FullPaper get_graph_get_paper(paper_id, fields)
Details about a paper

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paper_id** | **String** | The following types of IDs are supported: <ul>     <li><code>&lt;sha&gt;</code> - a Semantic Scholar ID, e.g. <code>649def34f8be52c8b66281af98ae884c09aef38b</code></li>     <li><code>CorpusId:&lt;id&gt;</code> - Semantic Scholar numerical ID, e.g. <code>215416146</code></li>     <li><code>DOI:&lt;doi&gt;</code> - a <a href=\"http://doi.org\">Digital Object Identifier</a>,         e.g. <code>DOI:10.18653/v1/N18-3011</code></li>     <li><code>ARXIV:&lt;id&gt;</code> - <a href=\"https://arxiv.org/\">arXiv.rg</a>, e.g. <code>ARXIV:2106.15928</code></li>     <li><code>MAG:&lt;id&gt;</code> - Microsoft Academic Graph, e.g. <code>MAG:112218234</code></li>     <li><code>ACL:&lt;id&gt;</code> - Association for Computational Linguistics, e.g. <code>ACL:W12-3903</code></li>     <li><code>PMID:&lt;id&gt;</code> - PubMed/Medline, e.g. <code>PMID:19872477</code></li>     <li><code>PMCID:&lt;id&gt;</code> - PubMed Central, e.g. <code>PMCID:2323736</code></li>     <li><code>URL:&lt;url&gt;</code> - URL from one of the sites listed below, e.g. <code>URL:https://arxiv.org/abs/2106.15928v1</code></li> </ul>  URLs are recognized from the following sites: <ul>     <li><a href=\"https://www.semanticscholar.org/\">semanticscholar.org</a></li>     <li><a href=\"https://arxiv.org/\">arxiv.org</a></li>     <li><a href=\"https://www.aclweb.org\">aclweb.org</a></li>     <li><a href=\"https://www.acm.org/\">acm.org</a></li>     <li><a href=\"https://www.biorxiv.org/\">biorxiv.org</a></li> </ul> | [required] |
**fields** | Option<**String**> | A comma-separated list of the fields to be returned.<br><br>  The following case-sensitive paper fields are recognized: <ul>     <li><code>paperId</code> - Always included</li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>title</code> - Included if no fields are specified</li>     <li><code>abstract</code></li>     <li><code>venue</code></li>     <li><code>year</code></li>     <li><code>referenceCount</code></li>     <li><code>citationCount</code></li>     <li><code>influentialCitationCount</code></li>     <li><code>isOpenAccess</code></li>     <li><code>fieldsOfStudy</code></li>     <li><code>s2FieldsOfStudy</code></li>     <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>     <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>     <li><code>journal</code> - Journal name, volume, and pages, if available</li>     <li><code>authors</code> - Up to 500 will be returned</li>         <ul>             <li><code>authorId</code> - Always included</li>             <li><code>externalIds</code></li>             <li><code>url</code></li>             <li><code>name</code> - Included if no fields are specified</li>             <li><code>aliases</code></li>             <li><code>affiliations</code></li>             <li><code>homepage</code></li>             <li><code>paperCount</code></li>             <li><code>citationCount</code></li>             <li><code>hIndex</code></li>                         <li>To get more detailed information about a paper's authors, use the <code>/paper/{paper_id}/authors</code> endpoint</li>         </ul>     <li><code>citations</code> - Up to 1000 will be returned</li>         <ul>             <li><code>paperId</code> - Always included</li>             <li><code>corpusId</code></li>             <li><code>externalIds</code></li>             <li><code>url</code></li>             <li><code>title</code> - Included if no fields are specified</li>             <li><code>abstract</code></li>             <li><code>venue</code></li>             <li><code>year</code></li>             <li><code>referenceCount</code></li>             <li><code>citationCount</code></li>             <li><code>influentialCitationCount</code></li>             <li><code>isOpenAccess</code></li>             <li><code>fieldsOfStudy</code></li>             <li><code>s2FieldsOfStudy</code></li>             <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>             <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>             <li><code>journal</code> - Journal name, volume, and pages, if available</li>             <li><code>authors</code> - Will include: <code>authorId</code> & <code>name</code></li>             <li>To get more detailed information about a paper's citations, use the <code>/paper/{paper_id}/citations</code> endpoint</li>         </ul>     <li><code>references</code> - Up to 1000 will be returned</li>         <ul>             <li><code>paperId</code> - Always included</li>             <li><code>externalIds</code></li>             <li><code>url</code></li>             <li><code>title</code> - Included if no fields are specified</li>             <li><code>abstract</code></li>             <li><code>venue</code></li>             <li><code>year</code></li>             <li><code>referenceCount</code></li>             <li><code>citationCount</code></li>             <li><code>influentialCitationCount</code></li>             <li><code>isOpenAccess</code></li>             <li><code>fieldsOfStudy</code></li>             <li><code>s2FieldsOfStudy</code></li>             <li><code>authors</code> - Will include: <code>authorId</code> & <code>name</code></li>             <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>             <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>             <li><code>journal</code> - Journal name, volume, and pages, if available</li>             <li>To get more detailed information about a paper's references, use the <code>/paper/{paper_id}/references</code> endpoint</li>         </ul>     <li><code>embedding</code> - Vector embedding of paper content from the <a href=\"https://github.com/allenai/specter\">SPECTER</a> model</li>     <li><code>tldr</code> - Auto-generated short summary of the paper from the <a href=\"https://github.com/allenai/scitldr\">SciTLDR</a> model</li> </ul> <br><br> Examples: <ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b</code></li>     <ul>         <li>Returns the paper's always included field of paperId plus its title.  </li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b?fields=url,year,authors</code></li>     <ul>         <li>Returns the paper's paperId, url, year, and list of authors.  </li>         <li>Each author has authorId and name.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b?fields=citations.authors</code></li>     <ul>         <li>Returns the paper's paperId and list of citations.  </li>         <li>Each citation has its paperId plus its list of authors.</li>         <li>Each author has their 2 always included fields of authorId and name.</li>     </ul> </ul> |  |

### Return type

[**crate::models::FullPaper**](FullPaper.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_graph_get_paper_authors

> crate::models::AuthorBatch get_graph_get_paper_authors(paper_id, offset, limit, fields)
Details about a paper's authors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paper_id** | **String** | The following types of IDs are supported: <ul>     <li><code>&lt;sha&gt;</code> - a Semantic Scholar ID, e.g. <code>649def34f8be52c8b66281af98ae884c09aef38b</code></li>     <li><code>CorpusId:&lt;id&gt;</code> - Semantic Scholar numerical ID, e.g. <code>215416146</code></li>     <li><code>DOI:&lt;doi&gt;</code> - a <a href=\"http://doi.org\">Digital Object Identifier</a>,         e.g. <code>DOI:10.18653/v1/N18-3011</code></li>     <li><code>ARXIV:&lt;id&gt;</code> - <a href=\"https://arxiv.org/\">arXiv.rg</a>, e.g. <code>ARXIV:2106.15928</code></li>     <li><code>MAG:&lt;id&gt;</code> - Microsoft Academic Graph, e.g. <code>MAG:112218234</code></li>     <li><code>ACL:&lt;id&gt;</code> - Association for Computational Linguistics, e.g. <code>ACL:W12-3903</code></li>     <li><code>PMID:&lt;id&gt;</code> - PubMed/Medline, e.g. <code>PMID:19872477</code></li>     <li><code>PMCID:&lt;id&gt;</code> - PubMed Central, e.g. <code>PMCID:2323736</code></li>     <li><code>URL:&lt;url&gt;</code> - URL from one of the sites listed below, e.g. <code>URL:https://arxiv.org/abs/2106.15928v1</code></li> </ul>  URLs are recognized from the following sites: <ul>     <li><a href=\"https://www.semanticscholar.org/\">semanticscholar.org</a></li>     <li><a href=\"https://arxiv.org/\">arxiv.org</a></li>     <li><a href=\"https://www.aclweb.org\">aclweb.org</a></li>     <li><a href=\"https://www.acm.org/\">acm.org</a></li>     <li><a href=\"https://www.biorxiv.org/\">biorxiv.org</a></li> </ul> | [required] |
**offset** | Option<**i32**> | When returning a list of results, start with the element at this position in the list.<br> The sum of <code>offset</code> and <code>limit</code> must be < 10000 |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return.<br> Must be <= 1000 |  |[default to 100]
**fields** | Option<**String**> | A comma-separated list of the fields to be returned.<br><br> The following case-sensitive author fields are recognized: <ul>     <li><code>authorId</code> - Always included</li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>name</code> - Included if no fields are specified</li>     <li><code>aliases</code></li>     <li><code>affiliations</code></li>     <li><code>homepage</code></li>     <li><code>paperCount</code></li>     <li><code>citationCount</code></li>     <li><code>hIndex</code></li>     <li><code>papers</code></li>     <ul>         <li><code>paperId</code> - Always included</li>         <li><code>externalIds</code></li>         <li><code>url</code></li>         <li><code>title</code> - Included if no fields are specified</li>         <li><code>abstract</code></li>         <li><code>venue</code></li>         <li><code>year</code></li>         <li><code>referenceCount</code></li>         <li><code>citationCount</code></li>         <li><code>influentialCitationCount</code></li>         <li><code>isOpenAccess</code></li>         <li><code>fieldsOfStudy</code></li>         <li><code>s2FieldsOfStudy</code></li>         <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>         <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>         <li><code>journal</code> - Journal name, volume, and pages, if available</li>         <li><code>authors</code> - Will include: <code>authorId</code> & <code>name</code></li>     </ul> </ul> <br><br> Examples: <ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/authors</code></li>     <ul>         <li>Returns with offset=0, and data is a list of all 3 authors.</li>         <li>Each author has their authorId and name</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/authors?fields=affiliations,papers&limit=2</code></li>     <ul>         <li>Returns with offset=0, next=2, and data is a list of 2 authors.</li>         <li>Each author has their authorId, affiliations, and list of papers.</li>         <li>Each paper has its paperId and title.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/authors?fields=url,papers.year,papers.authors&offset=2</code></li>     <ul>         <li>Returns with offset=2, and data is a list containing the last author.</li>         <li>This author has their authorId, url, and list of papers.</li>         <li>Each paper has its paperId, year, and list of authors.</li>         <li>In that list of authors, each author has their authorId and name.</li>     </ul> </ul> |  |

### Return type

[**crate::models::AuthorBatch**](AuthorBatch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_graph_get_paper_citations

> crate::models::CitationBatch get_graph_get_paper_citations(paper_id, offset, limit, fields)
Details about a paper's citations

Fetch details about the papers the cite this paper (i.e. papers in whose bibliography this paper appears)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paper_id** | **String** | The following types of IDs are supported: <ul>     <li><code>&lt;sha&gt;</code> - a Semantic Scholar ID, e.g. <code>649def34f8be52c8b66281af98ae884c09aef38b</code></li>     <li><code>CorpusId:&lt;id&gt;</code> - Semantic Scholar numerical ID, e.g. <code>215416146</code></li>     <li><code>DOI:&lt;doi&gt;</code> - a <a href=\"http://doi.org\">Digital Object Identifier</a>,         e.g. <code>DOI:10.18653/v1/N18-3011</code></li>     <li><code>ARXIV:&lt;id&gt;</code> - <a href=\"https://arxiv.org/\">arXiv.rg</a>, e.g. <code>ARXIV:2106.15928</code></li>     <li><code>MAG:&lt;id&gt;</code> - Microsoft Academic Graph, e.g. <code>MAG:112218234</code></li>     <li><code>ACL:&lt;id&gt;</code> - Association for Computational Linguistics, e.g. <code>ACL:W12-3903</code></li>     <li><code>PMID:&lt;id&gt;</code> - PubMed/Medline, e.g. <code>PMID:19872477</code></li>     <li><code>PMCID:&lt;id&gt;</code> - PubMed Central, e.g. <code>PMCID:2323736</code></li>     <li><code>URL:&lt;url&gt;</code> - URL from one of the sites listed below, e.g. <code>URL:https://arxiv.org/abs/2106.15928v1</code></li> </ul>  URLs are recognized from the following sites: <ul>     <li><a href=\"https://www.semanticscholar.org/\">semanticscholar.org</a></li>     <li><a href=\"https://arxiv.org/\">arxiv.org</a></li>     <li><a href=\"https://www.aclweb.org\">aclweb.org</a></li>     <li><a href=\"https://www.acm.org/\">acm.org</a></li>     <li><a href=\"https://www.biorxiv.org/\">biorxiv.org</a></li> </ul> | [required] |
**offset** | Option<**i32**> | When returning a list of results, start with the element at this position in the list.<br> The sum of <code>offset</code> and <code>limit</code> must be < 10000 |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return.<br> Must be <= 1000 |  |[default to 100]
**fields** | Option<**String**> | A comma-separated list of the fields to be returned. <br><br> The following case-sensitive citation fields are recognized: <ul>     <li><code>contexts</code></li>     <li><code>intents</code></li>     <li><code>isInfluential</code></li>     <li><code>paperId</code> - Always included</li>     <li><code>corpusId</code></li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>title</code> - Included if no fields are specified</li>     <li><code>abstract</code></li>     <li><code>venue</code></li>     <li><code>year</code></li>     <li><code>referenceCount</code></li>     <li><code>citationCount</code></li>     <li><code>influentialCitationCount</code></li>     <li><code>isOpenAccess</code></li>     <li><code>fieldsOfStudy</code></li>     <li><code>s2FieldsOfStudy</code></li>     <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>     <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>     <li><code>journal</code> - Journal name, volume, and pages, if available</li>     <li><code>authors</code> - Up to 500 will be returned.  Will include: <code>authorId</code> & <code>name</code></li> </ul> <br> <br> Examples: <ul>     <li>Let's suppose that the paper in the examples below has 1600 citations...</li>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/citations</code></li>     <ul>         <li>Returns with offset=0, next=100, and data is a list of 100 citations.</li>         <li>Each citation has a citingPaper which contains its paperId and title.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/citations?fields=contexts,intents,isInfluential,abstract&offset=200&limit=10</code></li>     <ul>         <li>Returns with offset=200, next=210, and data is a list of 10 citations.</li>         <li>Each citation has contexts, intents, isInfluential, and a citingPaper which contains its paperId and abstract.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/citations?fields=authors&offset=1500&limit=500</code></li>     <ul>         <li>Returns with offset=1500, and data is a list of the last 100 citations.</li>         <li>Each citation has a citingPaper which contains its paperId plus a list of authors</li>         <li>The authors under each citingPaper has their authorId and name.</li>     </ul> </ul> |  |

### Return type

[**crate::models::CitationBatch**](CitationBatch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_graph_get_paper_references

> crate::models::ReferenceBatch get_graph_get_paper_references(paper_id, offset, limit, fields)
Details about a paper's references

Fetch details about the papers cited by this paper (i.e. appearing in this paper's bibliography)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paper_id** | **String** | The following types of IDs are supported: <ul>     <li><code>&lt;sha&gt;</code> - a Semantic Scholar ID, e.g. <code>649def34f8be52c8b66281af98ae884c09aef38b</code></li>     <li><code>CorpusId:&lt;id&gt;</code> - Semantic Scholar numerical ID, e.g. <code>215416146</code></li>     <li><code>DOI:&lt;doi&gt;</code> - a <a href=\"http://doi.org\">Digital Object Identifier</a>,         e.g. <code>DOI:10.18653/v1/N18-3011</code></li>     <li><code>ARXIV:&lt;id&gt;</code> - <a href=\"https://arxiv.org/\">arXiv.rg</a>, e.g. <code>ARXIV:2106.15928</code></li>     <li><code>MAG:&lt;id&gt;</code> - Microsoft Academic Graph, e.g. <code>MAG:112218234</code></li>     <li><code>ACL:&lt;id&gt;</code> - Association for Computational Linguistics, e.g. <code>ACL:W12-3903</code></li>     <li><code>PMID:&lt;id&gt;</code> - PubMed/Medline, e.g. <code>PMID:19872477</code></li>     <li><code>PMCID:&lt;id&gt;</code> - PubMed Central, e.g. <code>PMCID:2323736</code></li>     <li><code>URL:&lt;url&gt;</code> - URL from one of the sites listed below, e.g. <code>URL:https://arxiv.org/abs/2106.15928v1</code></li> </ul>  URLs are recognized from the following sites: <ul>     <li><a href=\"https://www.semanticscholar.org/\">semanticscholar.org</a></li>     <li><a href=\"https://arxiv.org/\">arxiv.org</a></li>     <li><a href=\"https://www.aclweb.org\">aclweb.org</a></li>     <li><a href=\"https://www.acm.org/\">acm.org</a></li>     <li><a href=\"https://www.biorxiv.org/\">biorxiv.org</a></li> </ul> | [required] |
**offset** | Option<**i32**> | When returning a list of results, start with the element at this position in the list.<br> The sum of <code>offset</code> and <code>limit</code> must be < 10000 |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return.<br> Must be <= 1000 |  |[default to 100]
**fields** | Option<**String**> | A comma-separated list of the fields to be returned. <br><br> The following case-sensitive reference fields are recognized: <ul>     <li><code>contexts</code></li>     <li><code>intents</code></li>     <li><code>isInfluential</code></li>     <li><code>paperId</code> - Always included</li>     <li><code>corpusId</code></li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>title</code> - Included if no fields are specified</li>     <li><code>abstract</code></li>     <li><code>venue</code></li>     <li><code>year</code></li>     <li><code>referenceCount</code></li>     <li><code>citationCount</code></li>     <li><code>influentialCitationCount</code></li>     <li><code>isOpenAccess</code></li>     <li><code>fieldsOfStudy</code></li>     <li><code>s2FieldsOfStudy</code></li>     <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>     <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>     <li><code>journal</code> - Journal name, volume, and pages, if available</li>     <li><code>authors</code> - Up to 500 will be returned.  Will include: <code>authorId</code> & <code>name</code></li> </ul> <br> <br> Examples: <ul>     <li>Let's suppose that the paper in the examples below has 1600 references...</li>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/references</code></li>     <ul>         <li>Returns with offset=0, next=100, and data is a list of 100 references.</li>         <li>Each reference has a citedPaper which contains its paperId and title.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/references?fields=contexts,intents,isInfluential,abstract&offset=200&limit=10</code></li>     <ul>         <li>Returns with offset=200, next=210, and data is a list of 10 references.</li>         <li>Each reference has contexts, intents, isInfluential, and a citedPaper which contains its paperId and abstract.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/649def34f8be52c8b66281af98ae884c09aef38b/references?fields=authors&offset=1500&limit=500</code></li>     <ul>         <li>Returns with offset=1500, and data is a list of the last 100 references.</li>         <li>Each reference has a citedPaper which contains its paperId plus a list of authors</li>         <li>The authors under each citedPaper has their authorId and name.</li>     </ul> </ul> |  |

### Return type

[**crate::models::ReferenceBatch**](ReferenceBatch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_graph_get_paper_search

> crate::models::PaperSearchBatch get_graph_get_paper_search(query, fields, year, fields_of_study, offset, limit)
Search for papers by keyword

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | A plain-text search query string. No special query syntax is supported.  See our <a href=\"https://medium.com/ai2-blog/building-a-better-search-engine-for-semantic-scholar-ea23a0b661e7\">blog post</a> for a description of our search relevance algorithm.  Because of the subtleties of finding partial phrase matches in different parts of the document, be cautious about interpreting the <code>total</code> field as a count of documents containing any particular word in the query. | [required] |
**fields** | Option<**String**> | A comma-separated list of the fields to be returned.<br><br>  The following case-sensitive paper fields are recognized: <ul>     <li><code>paperId</code> - Always included</li>     <li><code>externalIds</code></li>     <li><code>url</code></li>     <li><code>title</code> - Included if no fields are specified</li>     <li><code>abstract</code></li>     <li><code>venue</code></li>     <li><code>year</code></li>     <li><code>referenceCount</code></li>     <li><code>citationCount</code></li>     <li><code>influentialCitationCount</code></li>     <li><code>isOpenAccess</code></li>     <li><code>fieldsOfStudy</code></li>     <li><code>s2FieldsOfStudy</code></li>     <li><code>publicationTypes</code> - Journal Article, Conference, Review, etc.</li>     <li><code>publicationDate</code> - YYYY-MM-DD, if available</li>     <li><code>journal</code> - Journal name, volume, and pages, if available</li>     <li><code>authors</code> - Up to 500 will be returned</li>     <ul>         <li><code>authorId</code> - Always included</li>         <li><code>name</code> - Always included</li>     </ul> </ul> <br><br> Examples: <ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/search?query=covid+vaccination&offset=100&limit=3</code></li>     <ul>         <li>Returns with total=576278, offset=100, next=103, and data is a list of 3 papers.</li>         <li>Each paper has the always included field of paperId plus its title.  </li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/search?query=covid&fields=url,abstract,authors</code></li>     <ul>         <li>Returns with total=639637, offset=0, next=100, and data is a list of 100 papers.</li>         <li>Each paper has paperId, url, abstract, and a list of authors.</li>         <li>Each author under that list has authorId and name.</li>     </ul>     <li><code>https://api.semanticscholar.org/graph/v1/paper/search?query=totalGarbageNonsense</code></li>     <ul>         <li>Returns with total = 0, offset=0, and data is a list of 0 papers.</li>     </ul> </ul> |  |
**year** | Option<**String**> | Restrict results to the given range of publication year (inclusive)<br> Examples: <ul>     <li><code>2019</code> in 2019</li>     <li><code>2016-2020</code> as early as 2016 or as late as 2020</li>     <li><code>2010-</code> during or after 2010</li>     <li><code>-2015</code> before or during 2015</li> </ul> |  |
**fields_of_study** | Option<**String**> | Restrict results to given field-of-study, using the `s2FieldsOfStudy` paper field.<br><br> Available fields are: <ul> <li>Computer Science</li> <li>Medicine</li> <li>Chemistry</li> <li>Biology</li> <li>Materials Science</li> <li>Physics</li> <li>Geology</li> <li>Psychology</li> <li>Art</li> <li>History</li> <li>Geography</li> <li>Sociology</li> <li>Business</li> <li>Political Science</li> <li>Economics</li> <li>Philosophy</li> <li>Mathematics</li> <li>Engineering</li> <li>Environmental Science</li> <li>Agricultural and Food Sciences</li> <li>Education</li> <li>Law</li> <li>Linguistics</li> </ul>  Use a comma-separated list to include papers from any of the listed fields<br> Example: <code>Physics,Mathematics</code> will return papers with either Physics or Mathematics in their list of fields-of-study. |  |
**offset** | Option<**i32**> | When returning a list of results, start with the element at this position in the list.<br> The sum of <code>offset</code> and <code>limit</code> must be < 10000 |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return.<br> Must be <= 100 |  |[default to 100]

### Return type

[**crate::models::PaperSearchBatch**](PaperSearchBatch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

