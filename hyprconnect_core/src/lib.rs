pub mod plugins;
pub mod protocol;
pub mod transport;

use anyhow::Result;

pub async fn ping(device_id: &str) -> Result<()> {
    println!("Ping enviado a dispositivo: {device_id}");
    Ok(())
}
