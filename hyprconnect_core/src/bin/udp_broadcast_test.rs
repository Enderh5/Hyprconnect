use anyhow::Result;
use hyprconnect_core::transport::udp::UdpBroadcast; // ajusta el nombre del crate si es distinto
use hyprconnect_core::transport::Broadcast;

fn main() -> Result<()> {
    // Creamos el broadcaster
    let udp_broadcast = UdpBroadcast::new("255.255.255.255:1716")?; // usa un puerto alto (no reservado)

    println!("🔵 Enviando broadcast de descubrimiento...");
    udp_broadcast.broadcast_probe()?;

    Ok(())
}
