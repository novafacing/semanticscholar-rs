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
pub struct ContentsOfThisBatch1 {
    /// List of contexts
    #[serde(rename = "contexts", skip_serializing_if = "Option::is_none")]
    pub contexts: Option<serde_json::Value>,
    /// List of intents
    #[serde(rename = "intents", skip_serializing_if = "Option::is_none")]
    pub intents: Option<serde_json::Value>,
    /// https://www.semanticscholar.org/faq#influential-citations
    #[serde(rename = "isInfluential", skip_serializing_if = "Option::is_none")]
    pub is_influential: Option<bool>,
    #[serde(rename = "citedPaper")]
    pub cited_paper: Box<crate::models::ReferenceCitedPaper>,
}

impl ContentsOfThisBatch1 {
    pub fn new(cited_paper: crate::models::ReferenceCitedPaper) -> ContentsOfThisBatch1 {
        ContentsOfThisBatch1 {
            contexts: None,
            intents: None,
            is_influential: None,
            cited_paper: Box::new(cited_paper),
        }
    }
}


