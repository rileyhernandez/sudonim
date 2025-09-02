mod arp;
mod artist;
mod device;

use crate::device::{DeviceRegistry, Device, get_user_input};
use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::{io::Write, process::Stdio};
use tokio::process::Command;
use spinners::{Spinner, Spinners};

#[tokio::main]
async fn main() -> Result<()> {
    let home_dir = std::env::home_dir().ok_or(anyhow!("No home directory"))?;
    let config_directory = home_dir.join(".config/sudonim");
    let config_file = config_directory.join("config.toml");
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            init_config(&config_directory, &config_file)?;
        }
        Commands::Scan { subnet } => {
            arp::nmap_scan(&subnet).await?;
            arp::neigh_show()
                .await?
                .iter()
                .for_each(|neighbor| println!("{neighbor}"))
        }
        _ => {
            // Load registry for device operations
            let mut registry = DeviceRegistry::load(&config_file)?;

            match cli.command {
                Commands::List => list_devices(&registry)?,
                Commands::Remove { device } => remove_device(&mut registry, &device, &config_file)?,
                Commands::New { subnet } => add_new_device(&mut registry, &config_file, &subnet).await?,
                Commands::Ssh { device } => ssh_to_device(&registry, &device).await?,
                Commands::Edit { device } => edit_device(&mut registry, &device)?,
                Commands::Rescan { device, subnet } => {
                    rescan_device(&mut registry, &device, &subnet, &config_file).await?;
                }
                Commands::Init | Commands::Scan { .. } => unreachable!(),
            }
        }
    }

    Ok(())
}

fn init_config(
    config_directory: &std::path::PathBuf,
    config_file: &std::path::PathBuf,
) -> Result<()> {
    let _ = std::fs::DirBuilder::new().create(config_directory);
    let mut file = match std::fs::File::create_new(config_file) {
        Ok(file) => Ok(file),
        Err(_) => {
            println!("Configuration already exists, would you like to overwrite it? [y/n]");
            let input = get_user_input("")?;
            match input.as_ref() {
                "y" | "Y" | "yes" | "Yes" => std::fs::File::create(config_file)
                    .map_err(|_| anyhow!("Failed to overwrite file")),
                _ => Err(anyhow!(
                    "User declined to overwrite existing configuration."
                )),
            }
        }
    }?;
    file.write_all(b"[devices]")?;
    println!("Config file initialized!");
    Ok(())
}

fn list_devices(registry: &DeviceRegistry) -> Result<()> {
    let devices = registry.devices.values();
    if devices.len() == 0 {
        println!("No devices found. Add new devices with \"new\" command.");
    } else {
        for device in devices {
            println!("{device}")
        }
    }
    Ok(())
}

fn remove_device(
    registry: &mut DeviceRegistry,
    device: &str,
    config_file: &std::path::PathBuf,
) -> Result<()> {
    registry
        .remove_device(device)
        .ok_or(anyhow!("Device not found"))?;
    registry.save(config_file)?;
    Ok(())
}



async fn add_new_device(
    registry: &mut DeviceRegistry,
    config_file: &std::path::PathBuf,
    subnet: &str
) -> Result<()> {
    let mut sp = Spinner::new(Spinners::Mindblown, "Scanning network for available devices...".into());
    // TODO: maybe only show devices not in registry?
    let _devices = arp::nmap_scan(subnet).await?;
    let neighbors = arp::neigh_show().await?;
    sp.stop();
    println!("");
    neighbors.iter().enumerate().for_each(|(index, neighbor)| {
        println!("{index}: {neighbor}");
    });
    println!("Select device to add:");
    let user_input = get_user_input("")?;
    println!("Enter new device's user:");
    let user = get_user_input("")?;
    println!("Enter name for new device (this is how you will identify it)");
    let name = get_user_input("")?;
    let index = user_input
        .parse::<usize>()
        .map_err(|_| anyhow!("Invalid index"))?;
    
    let selected_device: Device = neighbors
        .get(index)
        .map(|neighbor| neighbor.to_device(user, name))
        .ok_or_else(|| anyhow!("Invalid index"))?;
    registry.add_device(selected_device);
    registry.save(config_file)?;
    Ok(())
}

fn _add_new_device_from_input(
    registry: &mut DeviceRegistry,
    config_file: &std::path::PathBuf,
) -> Result<()> {
    registry.input()?;
    registry.save(config_file)?;
    Ok(())
}

async fn ssh_to_device(registry: &DeviceRegistry, device_name: &str) -> Result<()> {
    let device = registry
        .get_device(device_name)
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
    Ok(())
}

fn edit_device(registry: &mut DeviceRegistry, device: &str) -> Result<()> {
    registry.edit(device)?;
    Ok(())
}

async fn rescan_device(
    registry: &mut DeviceRegistry,
    device: &str,
    subnet: &str,
    config_file: &std::path::PathBuf,
) -> Result<()> {
    registry.rescan(device, subnet).await?;
    registry.save(config_file)?;
    Ok(())
}

#[derive(Parser)]
#[command(name = "sudonim")]
#[command(about = "A CLI tool for managing device IP addresses")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    List,
    Remove {
        #[arg(help = "Device name to remove")]
        device: String,
    },
    New {
        #[arg(help = "Subnet to scan")]
        subnet: String,
    },
    Ssh {
        #[arg(help = "Device name to SSH into")]
        device: String,
    },
    Edit {
        #[arg(help = "Device name to edit")]
        device: String,
    },
    Rescan {
        #[arg(help = "Device name to rescan")]
        device: String,
        #[arg(long, help = "Subnet to scan")]
        subnet: String,
    },
    Scan {
        #[arg(long, help = "Subnet to scan")]
        subnet: String,
    },
}
