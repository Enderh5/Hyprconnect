use clap::Parser;
use hyprconnect_core::ping;

#[derive(Parser, Debug)]
#[command(name = "hyprconnect")]
#[command(about = "CLI para interactuar con KDE Connect desde Hyprland")]
enum Command {
    Ping { device_id: String },
}

#[tokio::main]
async fn main() {
    let cmd = Command::parse();

    match cmd {
        Command::Ping { device_id } => {
            ping(&device_id).await;
        }
    }
}
