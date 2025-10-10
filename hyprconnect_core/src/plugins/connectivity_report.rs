use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityReport {
    #[serde(rename = "signalStrengths")]
    pub signal_strengths: HashMap<String, Signal>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signal {
    #[serde(rename = "networkType")]
    pub network_type: NetworkType,
    #[serde(rename = "signalStrenght")]
    pub signal_strength: SignalStrength,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // <-- coincide con 'GSM', 'CDMA', 'EDGE', etc.
pub enum NetworkType {
    GSM,
    CDMA,
    #[serde(rename = "iDEN")]
    IDEN,
    UMTS,
    CDMA2000,
    EDGE,
    GPRS,
    HSPA,
    LTE,
    #[serde(rename = "5G")]
    FiveG,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignalStrength {
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityReportRequest {}
