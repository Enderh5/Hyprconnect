use serde::{Deserialize, Serialize};

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
