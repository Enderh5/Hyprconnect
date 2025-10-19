pub mod udp;

use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use std::net::SocketAddr;

use crate::protocol::Packet;
use crate::transport::udp::LanBroadcast;

///Este trait deberá ser implementado por los distintos objetos que se encargen de enviar y recibir
///paquetes al dispositivo,
#[async_trait]
pub trait Transport: Send + Sync {
    ///Envía un paquete
    async fn send<T: Serialize + Send + Sync>(&self, packet: &Packet) -> Result<()>;

    ///Recibe un paquete (como JSON)
    async fn recieve(&self) -> Result<Packet>;

    /// Devuelve una descripción del medio (ej: "UDP", "Bluetooth", etc.)
    fn medium(&self) -> &'static str;
}

///Enum Wrapper para permitir seleccionar el tipo de transporte en tiempo de ejecución.
pub enum TransportKind {}

#[async_trait]
impl Transport for TransportKind {
    async fn send<T: Serialize + Send + Sync>(&self, _packet: &Packet) -> Result<()> {
        match self {
            _ => todo!(),
        }
    }

    async fn recieve(&self) -> Result<Packet> {
        match self {
            _ => todo!(),
        }
    }

    fn medium(&self) -> &'static str {
        match self {
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    pub id: String,
    pub name: String,
    pub addr: SocketAddr,
}

pub trait Broadcast {
    fn broadcast_probe(&self) -> Result<()>;
    /// Escuchar a broadcasts de otros clientes, la duración está en milisegundos
    fn listen_for_responses(&self, duration: u64) -> Result<Vec<Packet>>;
}

///Enum Wrapper para permitir seleccionar el tipo de transporte en tiempo de ejecución.
pub enum BroadcastKind {
    UdpBroadcast(LanBroadcast),
}

#[async_trait]
impl Broadcast for BroadcastKind {
    fn broadcast_probe(&self) -> Result<()> {
        match self {
            BroadcastKind::UdpBroadcast(udp_broadcast) => udp_broadcast.broadcast_probe(),
        }
    }
    fn listen_for_responses(&self, duration: u64) -> Result<Vec<Packet>> {
        match self {
            BroadcastKind::UdpBroadcast(udp_broadcast) => {
                udp_broadcast.listen_for_responses(duration)
            }
        }
    }
}
