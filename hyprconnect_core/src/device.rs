use anyhow::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use uuid::Uuid;

fn get_hostname() -> Result<String, Error> {
    // Alternativa: usar gethostname en Unix
    let mut buf = [0u8; 256];
    let len = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut i8, buf.len()) };
    if len != 0 {
        return Err(Error::from(io::Error::last_os_error()));
    }

    let device_name = std::str::from_utf8(&buf)
        .unwrap_or("unknown")
        .trim_end_matches(char::from(0))
        .to_string();
    Ok(device_name.to_string())
}

/// Devuelve la ruta donde se guarda el device_id
fn get_device_id_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().unwrap().join(".config"));
    base.join("hyprconnect").join("device_id.txt")
}

/// Genera un UUIDv4 compatible con KDE Connect
fn generate_device_id() -> String {
    Uuid::new_v4().to_string().replace("-", "_")
}

/// Guarda el device_id en disco
fn save_device_id(device_id: &str) -> io::Result<()> {
    let path = get_device_id_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(device_id.as_bytes())?;
    Ok(())
}

/// Carga el device_id existente o genera uno nuevo
pub fn load_or_generate_device_id() -> String {
    let path = get_device_id_path();

    if let Ok(contents) = fs::read_to_string(&path) {
        let id = contents.trim();
        if !id.is_empty() {
            return id.to_string();
        }
    }

    let id = generate_device_id();
    if let Err(e) = save_device_id(&id) {
        eprintln!("⚠️ No se pudo guardar device_id: {}", e);
    }
    id
}

/// Devuelve el hostname del sistema como device_name
pub fn load_device_name() -> String {
    get_hostname()
        .unwrap_or_else(|_| "Hyprconnect".into())
        .to_string()
        .into()
}
