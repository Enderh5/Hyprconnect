use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    #[serde(rename = "isCancel")]
    pub is_cancel: Option<bool>,
    #[serde(rename = "isClearable")]
    pub is_clearable: Option<bool>,
    #[serde(rename = "onlyOnce")]
    pub only_once: Option<bool>,
    pub silent: Option<bool>,
    pub time: Option<String>,
    #[serde(rename = "appName")]
    pub app_name: String,
    pub ticker: String,
    pub title: String,
    pub text: String,
    #[serde(rename = "payloadHash")]
    pub payload_hash: Option<String>,
    pub actions: Option<Vec<String>>,
    #[serde(rename = "requestReplyId")]
    pub request_reply_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationAction {
    pub key: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationReply {
    pub message: String,
    #[serde(rename = "requestReplyId")]
    pub request_reply_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequest {
    pub cancel: Option<String>,
    pub request: Option<bool>,
}
