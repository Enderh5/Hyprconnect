use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    protocol::{Identity, Packet, PacketBody},
    transport::Broadcast,
};
use anyhow::Result;
use serde_json::json;

pub struct LanBroadcast {
    address: SocketAddr,
    socket: UdpSocket,
}

impl LanBroadcast {
    pub fn new(address: &str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:1716")?;
        socket.set_broadcast(true)?;

        let broadcast_addr: SocketAddr = address.parse()?;

        Ok(Self {
            address: broadcast_addr,
            socket,
        })
    }
}

impl Broadcast for LanBroadcast {
    fn broadcast_probe(&self) -> Result<()> {
        let identity: PacketBody = PacketBody::Identity(Identity::new(HashMap::from([(
            "tcpPort".to_string(),
            json!(1716),
        )]))?);
        let packet = Packet::generate_packet(identity, None)?;
        let bytes = serde_json::to_vec(&packet)?;

        dbg!(packet);
        self.socket.send_to(&bytes, self.address)?;

        println!("📡 Broadcast enviado a {}", self.address);
        Ok(())
    }

    fn listen_for_responses(&self, duration: u64) -> Result<Vec<Packet>> {
        self.socket
            .set_read_timeout(Some(std::time::Duration::from_millis(duration)))?;

        let mut buf = [0u8; 2048];
        let mut responses: Vec<Packet> = Vec::new();

        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, _src)) => {
                    let _ = dbg!("{}", String::from_utf8(buf[..size].to_vec()));
                    match serde_json::from_slice(&buf[..size]) {
                        Ok(v) => responses.push(v),
                        Err(e) => print!("Error: {e}"),
                    };
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            }
        }

        Ok(responses)
    }
}
