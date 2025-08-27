mod arp;
mod artist;
mod device;

use crate::device::DeviceRegistry;
use anyhow::{Result, anyhow};
use clap::Parser;
use std::process::Stdio;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let home_dir = std::env::home_dir().ok_or(anyhow!("No home directory"))?;
    let config_file = home_dir.join(".config/sudonim/config.toml");
    let mut registry = DeviceRegistry::load(&config_file)?;
    let cli = Cli::parse();

    if cli.list {
        for device in registry.devices.values() {
            println!("Device: {device:?}");
        }
    }
    if let Some(device) = cli.remove {
        registry
            .remove_device(&device)
            .ok_or(anyhow!("Device not found"))?;
        registry.save(&config_file)?;
    }
    if cli.new {
        registry.input()?;
        registry.save(&config_file)?;
    }
    if let Some(device) = cli.ssh {
        let device = registry
            .get_device(&device)
            .ok_or(anyhow!("Device not found"))?;
        let mut cmd = Command::new("ssh");
        cmd.arg(device.to_address());
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        let mut child = cmd.spawn()?;
        let status = child.wait().await?;

        if !status.success() {
            eprintln!("SSH command exited with status: {}", status);
        }
    }

    // if let Some(Ok(Ok(res))) = set.join_next().await {
    //     println!("DEBUG: {res:?}");
    // }

    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    broadcast: Option<String>,
    #[clap(short, long)]
    arp: bool,
    #[arg(short, long)]
    timeout: Option<isize>,
    #[arg(long)]
    ssh: Option<String>,
    #[arg(long)]
    save: Option<String>,
    #[clap(short, long)]
    list: bool,
    #[arg(short, long)]
    remove: Option<String>,
    #[clap(short, long)]
    new: bool,
}

// #[derive(Debug)]
// enum EndCondition {
//     Broadcast,
//     Timeout,
//     Arp,
//     Ssh,
// }
