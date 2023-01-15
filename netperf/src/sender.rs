mod reader;
use crate::reader::config_reader::{Config,read_config};

use arrayvec::ArrayVec;
use std::io::{Read, Write};
use std::process::exit;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering, AtomicU16};
use std::time::{Instant, Duration};

use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;

/*
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::tcp::{TcpPacket,TcpFlags};
use pnet::packet::{Packet, MutablePacket};
*/

/*
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::{Ipv4};
use pnet::transport::{icmp_packet_iter, transport_channel};//, TransportReceiver, TransportSender};
*/

fn main() {
    
    //read config and retrieve data
    let config : Config = read_config("./config.toml");
    /*let local_addr: Ipv4Addr = match config.num_local{
        1 => config.ip1.parse().unwrap(),
        2 => config.ip2.parse().unwrap(),
        3 => config.ip3.parse().unwrap(),
        4 => config.ip3.parse().unwrap(),
        _ => {println!("Config Error :\nUnrecognized PC number :\"{}\", unable to continue.", config.num_local); exit(1)},
    };*/
    let dist_addr: Ipv4Addr = match config.num_dist{
        1 => config.ip1.parse().unwrap(),
        2 => config.ip2.parse().unwrap(),
        3 => config.ip3.parse().unwrap(),
        4 => config.ip3.parse().unwrap(),
        _ => {println!("Config Error :\nUnrecognized PC number :\"{}\", unable to continue.", config.num_dist); exit(1)},
    };

    let atomic_runner:Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let run_tcp:Arc<AtomicBool> = atomic_runner.clone();
    let run_ping:Arc<AtomicBool> = atomic_runner.clone();
    //let run_route:Arc<AtomicBool> = atomic_runner.clone();
    let run_print:Arc<AtomicBool> = atomic_runner.clone();
    let atomic_print_counter:Arc<AtomicU16> = Arc::new(AtomicU16::new(0));
    let print_count_tcp:Arc<AtomicU16> = atomic_print_counter.clone();
    let print_count_ping:Arc<AtomicU16> = atomic_print_counter.clone();
    //let print_count_route:Arc<AtomicU16> = atomic_print_counter.clone();
    let print_count_sync:Arc<AtomicU16> = atomic_print_counter.clone();

    let tcp_connection: thread::JoinHandle<()> = thread::Builder::new().name("TCP_thread".to_string()).spawn(move || {
        tcp_connection(dist_addr, config.port, run_tcp, print_count_tcp);
    }).unwrap();

    let icmp_ping: thread::JoinHandle<()> = thread::Builder::new().name("ICMP_ping_thread".to_string()).spawn(move || {
        icmp_ping(dist_addr, run_ping, print_count_ping);
    }).unwrap();

    let printer: thread::JoinHandle<()> = thread::Builder::new().name("printer_thread".to_string()).spawn(move || {
        printer(run_print, print_count_sync);
    }).unwrap();


    while atomic_runner.load(Ordering::SeqCst){
        let mut line = String::new();
        println!("WRITE \"exit\" to leave properly and get results");
        std::io::stdin().read_line(&mut line).unwrap();
        if line.starts_with("exit") {
            atomic_runner.store(false, Ordering::SeqCst);
        }
        println!("=====================EXITING=====================");
        thread::sleep(Duration::new(3, 0));
    }

    tcp_connection.join().expect("Erreur lors de la fermeture du thread TCP_thread");
    icmp_ping.join().expect("Erreur lors de la fermeture du thread ICMP_ping_thread");
    printer.join().expect("Erreur lors de la fermeture du thread printer_thread");

}


//********************************************************************************************************************************
// Fonction tcp connection --- sert à mesurer le débit moyen et la perte de packets. 
fn tcp_connection(dist_addr: Ipv4Addr, port: u16, run_tcp: Arc<AtomicBool>, print_count_tcp: Arc<AtomicU16>){
    let mut local_counter: u16 = 0;

    let distant_socket: SocketAddr = SocketAddr::new(IpAddr::V4(dist_addr), port);
    let mut stream: TcpStream = TcpStream::connect(distant_socket).unwrap();
    let mut array_vec: ArrayVec<u8, 65535> = ArrayVec::new();
        for _ in 0..array_vec.capacity() {
            array_vec.push(rand::random()); //INIT THE FUTURE WRITE BUFFER WITH FULL RANDOM VALUES (here packet size will be 65535 Bytes so 32kiB)
        }
    let write_buffer: [u8; 65535] = array_vec.into_inner().unwrap();
    let mut total_packets: u64 = 0;
    let start: Instant = Instant::now();
    let mut partial_total_packets: u64 = 0;
    let mut partial_start: Instant = Instant::now();
    let mut buff: [u8; 65535] = [0; 65535];

    while run_tcp.load(Ordering::SeqCst) { //MAIN LOOP OF THE THREAD
        let write_buffer_clone: [u8; 65535] = write_buffer.clone();
        stream.write(&write_buffer_clone).expect("Error while transmitting data from TCP socket.");
        stream.flush().unwrap();
        total_packets += 1; partial_total_packets += 1;
        
        let tmp_counter: u16 = print_count_tcp.load(Ordering::SeqCst);
        if tmp_counter!=local_counter {
            local_counter = tmp_counter.clone();
            stream.write("updatecall".as_bytes()).expect("Error while transmitting update call");
            stream.read(&mut buff).unwrap();
            let partial_time: u128 = partial_start.elapsed().as_millis();
            let partial_speed: f64 = (partial_total_packets as f64 * 65535f64 / 1000f64 / (partial_time as f64/1000f64)).round();
            let partial_receiver_count: u64 = String::from_utf8(buff.to_vec()).unwrap().trim_end_matches('\0').parse().unwrap();
            let partial_drop_count: u64 = partial_receiver_count-partial_total_packets;
            let partial_drop_ratio: f64 = ((partial_drop_count as f64 / partial_total_packets as f64)*100.0).round();
            println!(
                "Partial average speed : {}Ko/s\
                \nPartial packet drop ratio : {}% ({} dropped count/{})", 
                partial_speed, 
                partial_drop_ratio, 
                partial_drop_count, 
                partial_total_packets
            );
            partial_total_packets = 0;
            partial_start = Instant::now();
        }
    }

        //LAST PRINT BEFORE CLOSING
    stream.flush().unwrap();
    stream.write("finishcall".as_bytes()).expect("Error while transmitting finish call");
    stream.read(&mut buff).unwrap();
    let receiver_count: u64 = String::from_utf8(buff.to_vec()).unwrap().trim_end_matches('\0').parse().unwrap();
    let total_time:u64 = start.elapsed().as_secs();
    let total_speed: u64 = total_packets*65535/1000/total_time;
    let drop_count: u64 = receiver_count-total_packets;
    let drop_ratio: f64 = ((drop_count as f64 / total_packets as f64)*100.0).round();
 
    println!(
        "total time of the benchmark : {}\
        \ntotal bytes transfered : {}\
        \ntotal average speed : {} Ko/s\
        \nTotal packet drop ratio : {}% ({} dropped count/{} total)", 
        total_time,
        total_packets*65535,
        total_speed, 
        drop_ratio, 
        drop_count, 
        total_packets
    );

}

fn icmp_ping(dist_addr: Ipv4Addr, run_ping: Arc<AtomicBool>, print_count_ping: Arc<AtomicU16>){
    let mut local_counter: u16 = 0;

    let mut average_rtt: u64 = 0;
    let mut partial_average_rtt: u64 = 0;
    let mut ping_number: u64 = 0;
    let mut partial_ping_number: u64 = 0;

    let (pinger, results) = match Pinger::new(Some(500), Some(64)){
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e), 
    };

    pinger.add_ipaddr(dist_addr.to_string().as_str());

    pinger.run_pinger();

    while run_ping.load(Ordering::SeqCst) {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    println!("Ping out of time on: {}.", addr);
                }
                Receive { addr:_, rtt } => {
                    //println!("Receive from Address {} in {}ms.", addr, rtt.as_millis());
                    average_rtt = ((ping_number*average_rtt)+rtt.as_millis()as u64)/(ping_number+1);
                    partial_average_rtt = ((partial_ping_number*partial_average_rtt)+rtt.as_millis()as u64)/(partial_ping_number+1);
                    ping_number += 1;
                    partial_ping_number += 1;
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
        
        let tmp_counter: u16 = print_count_ping.load(Ordering::SeqCst);
        if tmp_counter!=local_counter {
            local_counter = tmp_counter.clone();
            println!("Partial average RTT: {}ms on {} pings", partial_average_rtt, partial_ping_number);
            partial_average_rtt=0;
            partial_ping_number=0;
        }    
    }
    //LAST PRINT 
    println!("FINAL average RTT: {}ms on {} pings", average_rtt, ping_number);
}

    /*let (mut tx, mut rx) = match transport_channel(4096, Layer4(Ipv4(IpNextHeaderProtocols::Icmp))) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("An error occurred when creating the transport channel: {}", e),
    };

    // Create an ICMP echo request packet
    let buff: [u8; 65535] = [0; 65535];
    let mut echo_request_packet: MutableEchoRequestPacket = MutableEchoRequestPacket::new(&mut buff).unwrap();
    echo_request_packet.set_icmp_type(IcmpTypes::EchoRequest);
    echo_request_packet.set_identifier(1);
    echo_request_packet.set_sequence_number(1);
    echo_request_packet.set_payload(&[1, 2, 3, 4]);
    // Send the packet
    match tx.send_to(echo_request_packet.packet().into(), IpAddr::V4(dist_addr)) {
        Ok(_) => println!("Sent an ICMP echo request packet"),
        Err(e) => panic!("An error occurred when sending the packet: {}", e),
    }

    // Receive the corresponding echo reply packet
    let mut iter = icmp_packet_iter(&mut rx);
    match iter.next() {
        Ok((packet, _)) => {
            let echo_reply_packet = packet.parse::<EchoReplyPacket>().unwrap();
            println!("Received an ICMP echo reply packet with identifier {} and sequence number {}",
                     echo_reply_packet.get_identifier(),
                     echo_reply_packet.get_sequence_number());
        }
        Err(e) => panic!("No ICMP echo reply packet was received, error : {}", e),
    }*/





// Function used to sychronise all printers to have them print periodic stats in the same time
fn printer(run_print: Arc<AtomicBool>, sync_count: Arc<AtomicU16>){
    let mut local_counter = sync_count.load(Ordering::SeqCst);
    let mut time: Instant = Instant::now();
    while run_print.load(Ordering::SeqCst) {
        if time.elapsed().as_millis()>3000{
            local_counter += 1;
            println!("\n\n\nprint counter : {}", local_counter);
            sync_count.store(local_counter, Ordering::SeqCst);
            time = Instant::now();
        }
    }
}








/*
fn drop_counter(){
    let mut count = 0;
    for interface in datalink::interfaces() {
        let handle = datalink::channel(&interface, Default::default()).unwrap();
        let mut iter = handle.iter();
        loop {
            match iter.next() {
                Ok(packet) => {
                    let packet = TcpPacket::new(packet).unwrap();
                    if packet.get_flags() & TcpFlags::RST != 0 {
                        count += 1;
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
    }
    println!("TCP packets that have been dropped and required a retransmission: {}", count);
}
*/
