use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MousepadEcho {
    pub key: Option<String>,
    #[serde(rename = "specialKey")]
    pub special_key: Option<f64>,
    pub alt: Option<bool>,
    pub ctrl: Option<bool>,
    pub shift: Option<bool>,
    #[serde(rename = "super")]
    pub super_key: Option<bool>,
    pub singleclick: Option<bool>,
    pub doubleclick: Option<bool>,
    pub middleclick: Option<bool>,
    pub rightclick: Option<bool>,
    pub singlehold: Option<bool>,
    pub singlerelease: Option<bool>,
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub scroll: Option<bool>,
    #[serde(rename = "isAck")]
    pub is_ack: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MousepadKeyboardState {
    pub state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MousepadRequest {
    pub key: Option<String>,
    #[serde(rename = "specialKey")]
    pub special_key: Option<f64>,
    pub alt: Option<bool>,
    pub ctrl: Option<bool>,
    pub shift: Option<bool>,
    #[serde(rename = "super")]
    pub super_key: Option<bool>,
    pub singleclick: Option<bool>,
    pub doubleclick: Option<bool>,
    pub middleclick: Option<bool>,
    pub rightclick: Option<bool>,
    pub singlehold: Option<bool>,
    pub singlerelease: Option<bool>,
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub scroll: Option<bool>,
    #[serde(rename = "sendAck")]
    pub send_ack: Option<bool>,
}
