use anyhow::{anyhow, Result};
use tokio::process::Command;

pub async fn nmap_scan(subnet: &str) -> Result<Vec<String>> {
    let output = Command::new("nmap").arg("-sn").arg(subnet).output().await?;
    let output = String::from_utf8(output.stdout)?;
    let devices = output
        .lines()
        .filter_map(|line| {
            if line.starts_with("Nmap scan report for ") {
                line.split_once("(")
                    .ok_or("Could not parse nmap line")
                    .map(|tuple| tuple.1)
                    .and_then(|ip| ip.split_once(")").ok_or("Could not parse nmap line"))
                    .map(|(ip, _)| ip.to_string())
                    .ok()
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    Ok(devices)
}
pub async fn neigh_show() -> Result<Vec<Neighbor>> {
    let output = Command::new("ip").arg("neigh").arg("show").output().await.map_err(|_| anyhow!("ip neigh cmd failed :("))?;
    let output = String::from_utf8(output.stdout)?;
    let neighbors = output
        .lines()
        .filter_map(|line| {
            Neighbor::new(line).ok()
        }).collect::<Vec<Neighbor>>();
    Ok(neighbors)
}

pub struct Neighbor {
    pub ip: String,
    pub mac: String,
}
impl Neighbor {
    fn new(neigbor: &str) -> Result<Self> {
        // TODO: handle edge cases, not quite sure what potential other outputs are...
        let mut parts = neigbor.split_whitespace();
        let ip = parts.next().ok_or(anyhow!("Missing IP address"))?;
        let _device = parts.next().ok_or(anyhow!("Missing device"))?;
        let _interface = parts.next().ok_or(anyhow!("Missing interface"))?;
        let _lladdr = parts.next().ok_or(anyhow!("Missing lladdr"))?;
        let mac = parts.next().ok_or(anyhow!("Missing MAC address"))?;
        let _status = parts.next().ok_or(anyhow!("Missing status"))?;
        
        // validate ip
        if ip.parse::<std::net::IpAddr>().is_err() {
            return Err(anyhow!("Invalid IP address"));
        }
        // validate mac (probably need more robust validation later)
        if mac.split(":").count() != 6 {
            return Err(anyhow!("Invalid MAC address"));
        }
        Ok(Neighbor { ip: ip.to_string(), mac: mac.to_string() })
    }
}

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
