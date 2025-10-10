use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelephonyBody {
    pub event: TelephonyEvent,

    pub contact_name: Option<String>,
    #[serde(rename = "messageBody")]
    pub message_body: Option<String>,

    pub phone_number: Option<String>,
    pub phone_thumbnail: Option<String>,
    pub is_cancel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TelephonyEvent {
    #[serde(rename = "missedCall")]
    MissedCall,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "talking")]
    Talking,
    #[serde(rename = "sms")]
    Sms,
}
