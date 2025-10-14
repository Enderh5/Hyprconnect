use std::{
    collections::HashMap,
    convert::identity,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    protocol::device::Device,
    protocol::{Identity, Packet, PacketBody},
    transport::Broadcast,
};
use anyhow::Result;
use serde_json::json;

pub struct UdpBroadcast {
    address: SocketAddr,
    socket: UdpSocket,
}

impl UdpBroadcast {
    pub fn new(address: &str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        let broadcast_addr: SocketAddr = address.parse()?;

        Ok(Self {
            address: broadcast_addr,
            socket,
        })
    }
}

impl Broadcast for UdpBroadcast {
    fn broadcast_probe(&self) -> Result<()> {
        let identity: PacketBody = PacketBody::Identity(Identity::new(HashMap::from([(
            "tcpPort".to_string(),
            json!(1234),
        )]))?);
        let packet = Packet::generate_packet(identity, None)?;
        let bytes = serde_json::to_vec(&packet)?;

        self.socket.send_to(&bytes, self.address)?;

        println!("📡 Broadcast enviado a {}", self.address);
        Ok(())
    }

    fn listen_for_responses(&self) -> Result<Vec<super::DiscoveredDevice>> {
        self.socket
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

        let mut buf = [0u8; 2048];
        let mut responses = Vec::new();

        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    let identity: Identity = serde_json::from_slice(&buf[..size]).unwrap();
                    responses.push(identity);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            }
        }

        Ok(identity)
    }
}
