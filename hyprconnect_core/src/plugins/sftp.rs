use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sftp {
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    pub ip: String,
    pub port: f64,
    pub user: String,
    pub password: String,
    pub path: String,
    #[serde(rename = "multi_paths")]
    pub multi_paths: Vec<String>,
    #[serde(rename = "pathNames")]
    pub path_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SftpRequest {
    #[serde(rename = "start_browsing")]
    start_browsing: bool,
}
