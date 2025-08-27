// use anyhow::{Result, anyhow};
// use tokio::process::Command;
//
// pub async fn arp_command() -> Result<Vec<Device>> {
//     let output = Command::new("arp").arg("-a").output().await?;
//     let output = String::from_utf8(output.stdout)?;
//     let devices = output
//         .lines()
//         .map(Device::from_string)
//         .collect::<Result<Vec<Device>>>()?;
//     Ok(devices)
// }
// pub async fn get_gateway(devices: &Vec<Device>) -> Result<&Device> {
//     if let Some(gateway) = devices.iter().next()
//         && gateway.hostname.starts_with("_gateway")
//     {
//         Ok(gateway)
//     } else {
//         Err(anyhow!("No gateway found"))
//     }
// }
//
// pub async fn broadcast_command(address: &str) -> Result<()> {
//     let _output = Command::new("ping").arg("-b").arg(address).output().await?;
//     Ok(())
// }
//
// #[derive(Debug)]
// pub struct Device {
//     hostname: String,
//     ip_address: String,
//     mac_address: String,
//     interface: String,
// }
// impl Device {
//     pub fn from_string(line: &str) -> Result<Self> {
//         let mut parts = line.split_whitespace();
//         let hostname = parts
//             .next()
//             .map(|s| s.to_string())
//             .ok_or(anyhow!("missing hostname"))?;
//         let ip_address = parts
//             .next()
//             .map(|s| s.to_string())
//             .ok_or(anyhow!("missing ip address"))?;
//         let _at = parts.next();
//         let mac_address = parts
//             .next()
//             .map(|s| s.to_string())
//             .ok_or(anyhow!("missing mac address"))?;
//         let _method = parts.next();
//         let _on = parts.next();
//         let interface = parts
//             .next()
//             .map(|s| s.to_string())
//             .unwrap_or("missing interface".parse()?);
//         Ok(Device {
//             hostname,
//             ip_address,
//             mac_address,
//             interface,
//         })
//     }
//     pub async fn check_vendor(&self) -> Result<String> {
//         let mac_address = &self.mac_address;
//         let url = format!("https://api.macvendors.com/{mac_address}");
//         let client = reqwest::Client::new();
//         let response = client.get(&url).send().await?;
//         if !response.status().is_success() {
//             return Err(anyhow!("Request failed with status: {}", response.status()));
//         }
//         let vendor = response.text().await?;
//         Ok(vendor)
//     }
//
// }
