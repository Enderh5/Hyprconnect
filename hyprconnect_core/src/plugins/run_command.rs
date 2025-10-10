use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunCommand {
    #[serde(rename = "commandList")]
    pub command_list: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunCommandRequest {
    pub key: Option<String>,

    #[serde(rename = "requestCommandList")]
    pub request_command_list: Option<String>,
}
