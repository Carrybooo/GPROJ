#![allow(unused)]

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub ip1: String,
    pub ip2: String,
    pub ip3: String,
    pub ip4: String,
    pub port: u16,
}

pub fn read_config() -> Config {
    let filename = "test.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    
    let data: Data = match toml::from_str(&contents) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    println!("{}", data.config.ip1); 
    println!("{}", data.config.ip2); 
    println!("{}", data.config.ip3); 
    println!("{}", data.config.ip4); 
    println!("{}", data.config.port); 

    return Config{
        ip1: data.config.ip1,
        ip2: data.config.ip2,
        ip3: data.config.ip3,
        ip4: data.config.ip4,
        port: data.config.port,
    };
}