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
pub struct PaperBatch {
    /// starting position for this batch
    #[serde(rename = "offset")]
    pub offset: i32,
    /// starting position of the next batch
    #[serde(rename = "next")]
    pub next: i32,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::ContentsOfThisBatch4>>,
}

impl PaperBatch {
    pub fn new(offset: i32, next: i32) -> PaperBatch {
        PaperBatch {
            offset,
            next,
            data: None,
        }
    }
}


