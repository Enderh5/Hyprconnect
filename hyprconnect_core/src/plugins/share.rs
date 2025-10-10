use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareRequest {
    filename: Option<String>,
    #[serde(rename = "creationTime")]
    creation_time: Option<f64>,
    #[serde(rename = "lastModified")]
    last_modified: Option<f64>,
    open: Option<bool>,
    #[serde(rename = "numberOfFiles")]
    number_of_files: Option<f64>,
    #[serde(rename = "totalPayloadSize")]
    total_payload_size: Option<f64>,
    text: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareRequestUpdate {
    #[serde(rename = "numberOfFiles")]
    number_of_files: Option<f64>,
    #[serde(rename = "totalPayloadSize")]
    total_payload_size: Option<f64>,
}
