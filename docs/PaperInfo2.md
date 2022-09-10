# PaperInfo2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**paper_id** | **String** |  | 
**external_ids** | Option<[**serde_json::Value**](.md)> | Other catalog IDs for this paper, if known. Supports ArXiv, MAG, ACL, PubMed, Medline, PubMedCentral, DBLP, DOI. | [optional]
**url** | Option<**String**> | URL on the Semantic Scholar website | [optional]
**title** | Option<**String**> |  | [optional]
**_abstract** | Option<**String**> |  | [optional]
**venue** | Option<**String**> |  | [optional]
**year** | Option<**i32**> |  | [optional]
**reference_count** | Option<**i32**> |  | [optional]
**citation_count** | Option<**i32**> |  | [optional]
**influential_citation_count** | Option<**i32**> |  | [optional]
**is_open_acess** | Option<**bool**> | https://www.openaccess.nl/en/what-is-open-access | [optional]
**fields_of_study** | Option<[**serde_json::Value**](.md)> | A list of high-level academic categories from external sources. | [optional]
**s2_fields_of_study** | Option<[**serde_json::Value**](.md)> | A list of high-level academic categories, inc their sources | [optional]
**authors** | Option<[**Vec<crate::models::AuthorInfo1>**](Author_Info_1.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


