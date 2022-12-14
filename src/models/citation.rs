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
pub struct Citation {
    /// List of contexts
    #[serde(rename = "contexts", skip_serializing_if = "Option::is_none")]
    pub contexts: Option<serde_json::Value>,
    /// List of intents
    #[serde(rename = "intents", skip_serializing_if = "Option::is_none")]
    pub intents: Option<serde_json::Value>,
    /// https://www.semanticscholar.org/faq#influential-citations
    #[serde(rename = "isInfluential", skip_serializing_if = "Option::is_none")]
    pub is_influential: Option<bool>,
    #[serde(rename = "citingPaper")]
    pub citing_paper: Box<crate::models::CitationCitingPaper>,
}

impl Citation {
    pub fn new(citing_paper: crate::models::CitationCitingPaper) -> Citation {
        Citation {
            contexts: None,
            intents: None,
            is_influential: None,
            citing_paper: Box::new(citing_paper),
        }
    }
}


