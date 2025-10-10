use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
struct ConnectPacket {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SmsAttatchmentFile {
    pub filename: String,
}

//SmsMessages Plugin
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SmsMessages {
    pub messages: Vec<Message>,
    pub version: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: f64,
    pub addresses: Vec<Address>,
    pub attachments: Vec<Attachment>,
    pub body: Option<String>,
    pub date: f64,
    pub event: Option<MessageEvent>,

    #[serde(
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_bool_to_int"
    )]
    pub read: Option<bool>,

    pub subid: Option<f64>,
    pub thread_id: f64,
    #[serde(rename = "type")]
    pub message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    All = 0,
    Inbox = 1,
    Sent = 2,
    Draft = 3,
    Outbox = 4,
    Failed = 5,
    Queued = 6,
}

#[derive(Debug)]
#[repr(u8)]
pub enum MessageEvent {
    Text = 1,
    MultiTarget = 2,
}
impl<'de> Deserialize<'de> for MessageEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            1 => Ok(Self::Text),
            2 => Ok(Self::MultiTarget),
            other => Err(serde::de::Error::custom(format!(
                "Unknown MessageEvent: {}",
                other
            ))),
        }
    }
}

impl Serialize for MessageEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            MessageEvent::Text => 1,
            MessageEvent::MultiTarget => 2,
        };
        serializer.serialize_u8(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub part_id: f64,
    pub mime_type: String,
    pub encoded_thumbnail: String,
    pub unique_identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRequest {
    pub addresses: Vec<Address>,
    pub attachments: Vec<OutgoingAttachment>,

    #[serde(rename = "messageBody")]
    pub message_body: String,
    #[serde(rename = "subId")]
    pub sub_id: Option<f64>,
    pub version: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingAttachment {
    pub filename: Option<String>,
    #[serde(rename = "base64EncodedFile")]
    pub base64_encoded_file: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsRequestAttachment {
    pub part_id: f64,
    pub unique_identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRequestConversation {
    #[serde(rename = "threadId")]
    pub thread_id: f64,

    #[serde(rename = "rangeStartTimestamp")]
    pub range_start_timestamp: Option<f64>,

    #[serde(rename = "numberToRequest")]
    pub number_to_request: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRequestConversations {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
}

//SystemVolume Plugin
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SystemVolume {
    #[serde(rename = "sinkList")]
    pub sink_list: Option<Vec<Stream>>,
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub muted: Option<bool>,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stream {
    pub name: String,
    pub description: String,
    pub enabled: Option<bool>,
    pub muted: bool,

    #[serde(rename = "maxVolume")]
    pub max_volume: Option<f64>,

    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SystemVolumeRequest {
    #[serde(rename = "requestSinks")]
    pub request_sinks: Option<bool>,
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub muted: Option<bool>,
    pub volume: Option<f64>,
}

//Telephony plugin
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

pub fn option_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    // Intenta deserializar un Option<u8>
    let opt = Option::<u8>::deserialize(deserializer)?;
    // Mapea Some(1) → Some(true), Some(0) → Some(false), None → None
    Ok(opt.map(|v| v != 0))
}

pub fn option_bool_to_int<S>(x: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(v) => serializer.serialize_u8(if *v { 1 } else { 0 }),
        None => serializer.serialize_none(),
    }
}
