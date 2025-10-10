use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    #[serde(rename = "currentCharge")]
    pub current_charge: f64,
    #[serde(rename = "isCharging")]
    pub is_charging: bool,

    #[serde(
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_bool_to_int",
        rename = "thresholdEvent"
    )]
    pub threshold_event: Option<bool>,
}

/// ⚠️ Deprecated: Use `kdeconnect.battery` instead.
/// This packet previously requested a battery update,
/// but its body is now empty.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[deprecated(
    since = "1.0.0",
    note = "This packet body is deprecated and should be empty"
)]
pub struct BatteryRequestBody {
    /// ⚠️ Deprecated: Field no longer used. If present, it's ignored.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(since = "1.0.0", note = "Field no longer used")]
    pub request: Option<bool>,
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
