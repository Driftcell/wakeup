use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::{SocketAddr, UdpSocket};
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "wakeup")]
#[command(about = "A Wake on LAN utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Wake up a host by name
    Wake {
        /// Host name to wake up
        host: String,
    },
    /// Add a new host configuration
    Add {
        /// Host name
        name: String,
        /// MAC address (format: XX:XX:XX:XX:XX:XX)
        mac: String,
        /// Broadcast address (optional, defaults to 255.255.255.255:9)
        #[arg(short, long)]
        broadcast: Option<String>,
    },
    /// Remove a host configuration
    Remove {
        /// Host name to remove
        name: String,
    },
    /// List all configured hosts
    List,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    hosts: HashMap<String, HostConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HostConfig {
    mac: String,
    broadcast: String,
}

impl Config {
    fn load() -> Result<Self> {
        let config_path = Self::config_path();
        if !config_path.exists() {
            return Ok(Config::default());
        }
        
        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        let content = toml::to_string(self)
            .context("Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    fn config_path() -> std::path::PathBuf {
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("config.toml")
    }
}

fn parse_mac_address(mac: &str) -> Result<[u8; 6]> {
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() != 6 {
        anyhow::bail!("MAC address must have 6 parts separated by colons");
    }
    
    let mut bytes = [0u8; 6];
    for (i, part) in parts.iter().enumerate() {
        bytes[i] = u8::from_str_radix(part, 16)
            .with_context(|| format!("Invalid hex value: {}", part))?;
    }
    
    Ok(bytes)
}

fn create_magic_packet(mac: [u8; 6]) -> Vec<u8> {
    let mut packet = Vec::new();
    
    // Add 6 bytes of 0xFF
    packet.extend_from_slice(&[0xFF; 6]);
    
    // Add the MAC address 16 times
    for _ in 0..16 {
        packet.extend_from_slice(&mac);
    }
    
    packet
}

fn send_wake_on_lan(mac: &str, broadcast_addr: &str) -> Result<()> {
    let mac_bytes = parse_mac_address(mac)?;
    let packet = create_magic_packet(mac_bytes);
    
    let socket = UdpSocket::bind("0.0.0.0:0")
        .context("Failed to bind UDP socket")?;
    
    socket.set_broadcast(true)
        .context("Failed to set broadcast option")?;
    
    let addr: SocketAddr = broadcast_addr.parse()
        .with_context(|| format!("Invalid broadcast address: {}", broadcast_addr))?;
    
    socket.send_to(&packet, addr)
        .context("Failed to send magic packet")?;
    
    println!("Magic packet sent to {} ({})", mac, broadcast_addr);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;
    
    match cli.command {
        Commands::Wake { host } => {
            match config.hosts.get(&host) {
                Some(host_config) => {
                    send_wake_on_lan(&host_config.mac, &host_config.broadcast)?;
                    println!("Wake-on-LAN packet sent to host '{}'", host);
                }
                None => {
                    println!("Host '{}' not found in configuration.", host);
                    println!("Use 'wakeup add' to add a new host.");
                    std::process::exit(1);
                }
            }
        }
        Commands::Add { name, mac, broadcast } => {
            // Validate MAC address format
            parse_mac_address(&mac)
                .with_context(|| format!("Invalid MAC address format: {}", mac))?;
            
            let broadcast_addr = broadcast.unwrap_or_else(|| "255.255.255.255:9".to_string());
            
            // Validate broadcast address
            let _: SocketAddr = broadcast_addr.parse()
                .with_context(|| format!("Invalid broadcast address: {}", broadcast_addr))?;
            
            config.hosts.insert(name.clone(), HostConfig {
                mac: mac.clone(),
                broadcast: broadcast_addr.clone(),
            });
            
            config.save()?;
            println!("Host '{}' added with MAC {} and broadcast {}", name, mac, broadcast_addr);
        }
        Commands::Remove { name } => {
            if config.hosts.remove(&name).is_some() {
                config.save()?;
                println!("Host '{}' removed from configuration", name);
            } else {
                println!("Host '{}' not found in configuration", name);
            }
        }
        Commands::List => {
            if config.hosts.is_empty() {
                println!("No hosts configured.");
            } else {
                println!("Configured hosts:");
                for (name, host_config) in &config.hosts {
                    println!("  {} -> {} (broadcast: {})", name, host_config.mac, host_config.broadcast);
                }
            }
        }
    }
    
    Ok(())
}
