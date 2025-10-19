use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::Result;
use hyprconnect_core::protocol::{Identity, Packet, PacketBody};
use hyprconnect_core::transport::udp::LanBroadcast; // ajusta el nombre del crate si es distinto
use hyprconnect_core::transport::Broadcast;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<()> {
    // Creamos el broadcaster
    let listener = TcpListener::bind("0.0.0.0:1716").await?;
    println!("Esperando conexiones TCP de KDE Connect...");
    let udp_broadcast = LanBroadcast::new("255.255.255.255:1716")?; // usa un puerto alto (no reservado)

    println!("🔵 Enviando broadcast de descubrimiento...");
    udp_broadcast.broadcast_probe()?;

    let a = udp_broadcast.listen_for_responses(1000);
    let _ = dbg!(a);

    // HashMap compartido para guardar streams
    let connections: Arc<Mutex<HashMap<String, tokio::net::TcpStream>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let start_time = Instant::now();

    loop {
        if start_time.elapsed() > Duration::from_secs(10) {
            println!("10 segundos transcurridos, saliendo del loop.");
            break;
        }
        let (mut stream, addr) = listener.accept().await?;
        let peer = addr.to_string();
        println!("Conexión desde {}", peer);

        // Enviar identidad
        let identity = Packet::generate_packet(
            PacketBody::Identity(Identity::new(HashMap::from([(
                "tcpPort".to_string(),
                json!(1716),
            )]))?),
            None,
        )?;
        let mut data = serde_json::to_vec(&identity)?;
        data.push(b'\n');
        stream.write_all(&data).await?;

        // Guardar la conexión en un HashMap y mantenerla viva
        let connections = Arc::clone(&connections);
        let stream_clone = Arc::clone(&stream);
        tokio::spawn(async move {
            connections.lock().unwrap().insert(peer.clone(), stream);
            // Loop de lectura para mantener TCP activo
            loop {
                let mut buf = [0u8; 4096];
                let mut locked = stream_clone.lock().unwrap();
                match locked.readable().await {
                    Ok(_) => match stream.try_read(&mut buf) {
                        Ok(0) => break, // el cliente cerró
                        Ok(n) => println!("Recibí {} bytes de {}", n, peer),
                        Err(_) => continue,
                    },
                    Err(_) => break,
                }
            }
            connections.lock().unwrap().remove(&peer);
        });
    }

    println!("Conexiones activas:");
    for peer in connections.lock().unwrap().keys() {
        println!("- {}", peer);
    }

    Ok(())
}
