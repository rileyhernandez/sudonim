use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::PathBuf;

use crate::arp::neigh_show;
use crate::arp::nmap_scan;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub user: String,
    pub ip_address: String,
    pub mac_address: String,
    pub name: String,
}
impl Default for Device {
    fn default() -> Self {
        Self {
            user: "server".into(),
            ip_address: "192.168.1.7".into(),
            mac_address: "d8:3a:dd:70:5f:62".into(),
            name: "Mingus".into(),
        }
    }
}
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Device: {}\n  User: {}\n  IP: {}\n  MAC: {}",
            self.name, self.user, self.ip_address, self.mac_address
        )
    }
}
impl Device {
    pub fn to_address(&self) -> String {
        format!("{}@{}", self.user, self.ip_address)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeviceRegistry {
    pub devices: HashMap<String, Device>,
}
impl DeviceRegistry {
    pub fn load(path: &PathBuf) -> Result<Self> {
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(_e) => Err(anyhow!(
                "Failed to load config from path: {path:?} \nMust run \"--init\" to setup."
            )),
        }
    }
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    pub fn add_device(&mut self, device: Device) {
        let name = device.name.clone();
        self.devices.insert(name, device);
    }
    pub fn remove_device(&mut self, name: &str) -> Option<Device> {
        self.devices.remove(name)
    }
    pub fn get_device(&self, name: &str) -> Option<&Device> {
        self.devices.get(name)
    }
    pub fn input(&mut self) -> Result<()> {
        println!("Enter device user:");
        let user = get_user_input("")?;
        println!("Enter device IP address:");
        let ip_address = get_user_input("")?;
        println!("Enter device MAC address:");
        let mac_address = get_user_input("")?;
        println!("Create name for device:");
        let name = get_user_input("")?;
        let device = Device {
            user,
            ip_address,
            mac_address,
            name,
        };
        self.add_device(device);
        Ok(())
    }
    pub fn edit(&mut self, name: &str) -> Result<()> {
        if let Some(device) = self.devices.get_mut(name) {
            println!("Editing: {name}");
            println!("Enter new IP Address:");
            let new_ip = get_user_input("")?;
            device.ip_address = new_ip;
            println!("IP Address updated successfully!");
            Ok(())
        } else {
            Err(anyhow!("Device not found"))
        }
    }
    pub async fn rescan(&mut self, device: &str) -> Result<()> {
        let device = self
            .devices
            .get_mut(device)
            .ok_or(anyhow!("Device not found"))?;
        println!("Old IP Address: {}", device.ip_address);
        println!("Searching subnet for device with MAC address {}...", device.mac_address);
        let _map = nmap_scan("192.168.0.0/24").await?;
        let neighbors = neigh_show().await?;
        device.ip_address = neighbors
            .iter()
            .find(|neighbor| neighbor.mac == device.mac_address)
            .ok_or(anyhow!("Device not found"))?
            .ip
            .clone();
        println!("New IP Address: {}", device.ip_address);
        Ok(())
    }
}

fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| anyhow!("Failed to read input"))?;
    input.truncate(input.len() - 1);
    Ok(input)
}
