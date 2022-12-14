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
pub struct Error404 {
    /// Depending on the case, error message may be any of these: <ul>     <li><code>\"Paper/Author/Object not found\"</code></li>     <li><code>\"Paper/Author/Object with id ### not found\"</code></li> </ul>
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Error404 {
    pub fn new() -> Error404 {
        Error404 {
            error: None,
        }
    }
}


