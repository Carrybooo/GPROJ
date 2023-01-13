#![allow(unused)]
#[macro_use]

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

mod reader;
use crate::reader::config_reader::{Config,Data,read_config};

fn main() {
    
    let config : Config = read_config();

    let addr: Ipv4Addr = config.ip1.parse().unwrap();
    let ip_adrr = IpAddr::V4(addr);
    let socket_addr = SocketAddr::new(ip_adrr, 10080);
}

