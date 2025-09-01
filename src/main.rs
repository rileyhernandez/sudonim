mod arp;
mod artist;
mod device;

use crate::device::DeviceRegistry;
use anyhow::{Result, anyhow};
use clap::Parser;
use std::{io::Write, process::Stdio};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let home_dir = std::env::home_dir().ok_or(anyhow!("No home directory"))?;
    let config_directory = home_dir.join(".config/sudonim");
    let config_file = config_directory.join("config.toml");

    let cli = Cli::parse();
    if cli.init {
        let _ = std::fs::DirBuilder::new()
            .create(config_directory)
            .map_err(|_| anyhow!("Failed to make config directory."));
        let mut file = std::fs::File::create_new(&config_file)
            .map_err(|_| anyhow!("Failed to create config file."))?;
        file.write_all(b"[devices]")?;
        println!("Config file initialized!")
    }

    let mut registry = DeviceRegistry::load(&config_file)?;

    if cli.list {
        for device in registry.devices.values() {
            println!("{device}");
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
    if let Some(device) = cli.edit {
        registry.edit(&device)?;
    }
    if let Some(device) = cli.rescan {
        registry.rescan(&device).await?;
        registry.save(&config_file)?;
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
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
    #[arg(short, long)]
    edit: Option<String>,
    #[clap(short, long)]
    init: bool,
    #[arg(long)]
    rescan: Option<String>,
}