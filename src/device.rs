use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

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
impl Device {
    pub fn to_address(&self) -> String {
        format!("{}@{}", self.user, self.ip_address)
    }
    pub fn from_address(device: &str) -> Result<Self> {
        let mut split = device.split('@');
        let user = split.next().ok_or(anyhow!("No user"))?;
        let ip_address = split.next().ok_or(anyhow!("No ip address"))?;
        Ok(Self {
            user: user.into(),
            ip_address: ip_address.into(),
            ..Default::default()
        })
    }
    // pub fn read_from_config(config_path: &PathBuf) -> Result<Self> {
    //     let config_str = std::fs::read_to_string(config_path)?;
    //     let
    // }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeviceRegistry {
    pub devices: HashMap<String, Device>,
}
impl DeviceRegistry {
    pub fn load(path: &PathBuf) -> Result<Self> {
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(e) => Err(anyhow!("this failed ig")),
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
    pub fn list_devices(&self) -> Vec<&Device> {
        self.devices.values().collect()
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
