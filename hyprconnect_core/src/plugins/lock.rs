use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockRequest {
    #[serde(rename = "requestLocked")]
    pub request_locked: Option<bool>,
    #[serde(rename = "setLocked")]
    pub set_locked: Option<bool>,
}
