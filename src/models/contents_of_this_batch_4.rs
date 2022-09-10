/*
 * Academic Graph API
 *
 * Fetch paper and author data from the Semantic Scholar Academic Graph (S2AG)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentsOfThisBatch4 {
    #[serde(rename = "paperId")]
    pub paper_id: String,
    /// Other catalog IDs for this paper, if known. Supports ArXiv, MAG, ACL, PubMed, Medline, PubMedCentral, DBLP, DOI.
    #[serde(rename = "externalIds", skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<serde_json::Value>,
    /// URL on the Semantic Scholar website
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "abstract", skip_serializing_if = "Option::is_none")]
    pub _abstract: Option<String>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "referenceCount", skip_serializing_if = "Option::is_none")]
    pub reference_count: Option<i32>,
    #[serde(rename = "citationCount", skip_serializing_if = "Option::is_none")]
    pub citation_count: Option<i32>,
    /// https://www.semanticscholar.org/faq#influential-citations
    #[serde(rename = "influentialCitationCount", skip_serializing_if = "Option::is_none")]
    pub influential_citation_count: Option<i32>,
    /// https://www.openaccess.nl/en/what-is-open-access
    #[serde(rename = "isOpenAccess", skip_serializing_if = "Option::is_none")]
    pub is_open_access: Option<bool>,
    /// A list of high-level academic categories from external sources.
    #[serde(rename = "fieldsOfStudy", skip_serializing_if = "Option::is_none")]
    pub fields_of_study: Option<serde_json::Value>,
    /// A list of high-level academic categories, inc their sources
    #[serde(rename = "s2FieldsOfStudy", skip_serializing_if = "Option::is_none")]
    pub s2_fields_of_study: Option<serde_json::Value>,
    /// The type of this publication
    #[serde(rename = "publicationTypes", skip_serializing_if = "Option::is_none")]
    pub publication_types: Option<Vec<String>>,
    /// Year-month-day when this paper was published
    #[serde(rename = "publicationDate", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    /// Journal name, volume, and pages
    #[serde(rename = "journal", skip_serializing_if = "Option::is_none")]
    pub journal: Option<serde_json::Value>,
    #[serde(rename = "authors", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<crate::models::AuthorInfo>>,
    #[serde(rename = "citations", skip_serializing_if = "Option::is_none")]
    pub citations: Option<Vec<crate::models::PaperInfo>>,
    #[serde(rename = "references", skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<crate::models::PaperInfo1>>,
}

impl ContentsOfThisBatch4 {
    pub fn new(paper_id: String) -> ContentsOfThisBatch4 {
        ContentsOfThisBatch4 {
            paper_id,
            external_ids: None,
            url: None,
            title: None,
            _abstract: None,
            venue: None,
            year: None,
            reference_count: None,
            citation_count: None,
            influential_citation_count: None,
            is_open_access: None,
            fields_of_study: None,
            s2_fields_of_study: None,
            publication_types: None,
            publication_date: None,
            journal: None,
            authors: None,
            citations: None,
            references: None,
        }
    }
}


