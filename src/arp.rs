use crate::device::Device;
use anyhow::{Result, anyhow};
use std::fmt;
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
    let output = Command::new("ip")
        .arg("neigh")
        .arg("show")
        .output()
        .await
        .map_err(|_| anyhow!("ip neigh cmd failed :("))?;
    let output = String::from_utf8(output.stdout)?;
    let neighbors = output
        .lines()
        .filter_map(|line| Neighbor::new(line).ok())
        .collect::<Vec<Neighbor>>();
    Ok(neighbors)
}

pub struct Neighbor {
    pub ip: String,
    pub mac: String,
}
impl Neighbor {
    fn new(neigbor: &str) -> Result<Self> {
        let mut parts = neigbor.split_whitespace();
        let ip = parts.next().ok_or(anyhow!("Missing IP address"))?;
        let _device = parts.next().ok_or(anyhow!("Missing device"))?;
        let _interface = parts.next().ok_or(anyhow!("Missing interface"))?;
        let _lladdr = parts.next().ok_or(anyhow!("Missing lladdr"))?;
        let mac = parts.next().ok_or(anyhow!("Missing MAC address"))?;
        let _status = parts.next().ok_or(anyhow!("Missing status"))?;
        Ok(Neighbor {
            ip: ip.to_string(),
            mac: mac.to_string(),
        })
    }
    pub fn to_device(&self, user: String, name: String) -> Device {
        Device::new(user, self.ip.clone(), self.mac.clone(), name)
    }
}
impl fmt::Display for Neighbor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.ip, self.mac)
    }
}