use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Packet<T> {
    id: f64,
    #[serde(rename = "type")]
    packet_type: String,
    body: T,
    #[serde(rename = "payloadSize")]
    payload_size: f64,
    #[serde(default)]
    pub payload_transfer_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identity {
    #[serde(rename = "deviceId")]
    device_id: String,

    #[serde(rename = "deviceName")]
    device_name: String,
    #[serde(rename = "device_type")]
    device_type: String,

    #[serde(rename = "incomingCapabilities")]
    incoming_capabilities: Vec<String>,
    #[serde(rename = "outgoingCapabilities")]
    outgoing_capabilities: Vec<String>,

    #[serde(rename = "protocolVersion")]
    protocol_version: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pair {
    pair: bool,
}
