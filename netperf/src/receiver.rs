mod reader;
use crate::reader::config_reader::{Config,read_config};

use std::io::{Read, Write};
use std::process::exit;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

fn main() {
    let config : Config = read_config("./config.toml");
    let local_addr: Ipv4Addr = match config.num_local{
        1 => config.ip1.parse().unwrap(),
        2 => config.ip2.parse().unwrap(),
        3 => config.ip3.parse().unwrap(),
        4 => config.ip3.parse().unwrap(),
        _ => {println!("Config Error :\nUnrecognized PC number :\"{}\", unable to continue.", config.num_local); exit(1)},
    };
    let dist_addr: Ipv4Addr = match config.num_dist{
        1 => config.ip1.parse().unwrap(),
        2 => config.ip2.parse().unwrap(),
        3 => config.ip3.parse().unwrap(),
        4 => config.ip3.parse().unwrap(),
        _ => {println!("Config Error :\nUnrecognized PC number :\"{}\", unable to continue.", config.num_dist); exit(1)},
    };

    println!("Local address: {}\nDistant address: {}\nPort: {}", local_addr, dist_addr, config.port);

    let local_socket: SocketAddr = SocketAddr::new(IpAddr::V4(local_addr), config.port);


    let listener: TcpListener = TcpListener::bind(local_socket).unwrap();

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut received_packets: u64 = 0;
    let mut partial_packets: u64 = 0;
    let peer_addr = stream.peer_addr().unwrap().to_string();
    println!("Connection started by this remote address: {}", peer_addr);
    let mut buf: [u8; 65535] = [0; 65535];
    loop{
        let bytes_read: usize = stream.read(&mut buf).unwrap_or_default();
        //println!("bytes_read: {}", bytes_read);
        received_packets += 1;
        partial_packets += 1;
        if bytes_read == 0 {
            println!("Connection closed with this address: {}", peer_addr);
            break;
        }
        let copy = buf.clone();
        let received_data = String::from_utf8_lossy(&copy);
        if received_data.len()>5 {
            if received_data.contains("updatecall") {
                println!("update call received, sending count: {}", partial_packets);
                stream.write(partial_packets.to_string().as_bytes()).expect("Error while sending final count of received packets");
                stream.flush().unwrap();
                partial_packets = 0;
                buf = [0; 65535];
            }
            if received_data.starts_with("finishcall") {
                println!("finish call received, sending total count: {}", received_packets);
                stream.write(received_packets.to_string().as_bytes()).expect("Error while sending final count of received packets");
                stream.flush().unwrap();
                break;
            }
        }
    }
}