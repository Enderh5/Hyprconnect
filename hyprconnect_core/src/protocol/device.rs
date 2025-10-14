use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

use crate::protocol::{Identity, Packet, PacketBody};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    /// "phone" o "desktop"
    pub device_type: String,
    pub incoming_capabilities: Vec<String>,
    pub outgoing_capabilities: Vec<String>,
}

impl TryFrom<Packet> for Device {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let PacketBody::Identity(b) = value.body {
            Ok(Self {
                device_id: b.device_id,
                device_name: b.device_name,
                device_type: b.device_type,
                incoming_capabilities: b.incoming_capabilities,
                outgoing_capabilities: b.outgoing_capabilities,
            })
        } else {
            Err(anyhow!(
                "Solo se puede obtener un objeto dispositivo de un paquete Identity."
            ))
        }
    }
}
