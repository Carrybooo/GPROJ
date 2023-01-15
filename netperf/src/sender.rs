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

/*
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::tcp::{TcpPacket,TcpFlags};
use pnet::packet::{Packet, MutablePacket};
*/

fn main() {
    
    //read config and retrieve data
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

    let atomic_runner:Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let run_tcp:Arc<AtomicBool> = atomic_runner.clone();
    //let run_ping:Arc<AtomicBool> = atomic_runner.clone();
    //let run_route:Arc<AtomicBool> = atomic_runner.clone();
    let run_print:Arc<AtomicBool> = atomic_runner.clone();
    let atomic_counter:Arc<AtomicU16> = Arc::new(AtomicU16::new(0));
    let count_tcp:Arc<AtomicU16> = atomic_counter.clone();
    //let count_tcp:Arc<AtomicU16> = atomic_counter.clone();
    //let count_tcp:Arc<AtomicU16> = atomic_counter.clone();
    let count_print:Arc<AtomicU16> = atomic_counter.clone();

    let tcp_connection: thread::JoinHandle<()> = thread::Builder::new().name("TCP_thread".to_string()).spawn(move || {
        tcp_connection(dist_addr, config.port, run_tcp, count_tcp);
    }).unwrap();

    let printer: thread::JoinHandle<()> = thread::Builder::new().name("printer_thread".to_string()).spawn(move || {
        printer(run_print, count_print);
    }).unwrap();


    while atomic_runner.load(Ordering::SeqCst){
        let mut line = String::new();
        println!("WRITE \"exit\" to leave properly and get results");
        std::io::stdin().read_line(&mut line).unwrap();
        if line.starts_with("exit") {
            atomic_runner.store(false, Ordering::SeqCst);
        }
        thread::sleep(Duration::new(3, 0));
    }

    tcp_connection.join().expect("Erreur lors de la fermeture du thread TCP_thread");
    printer.join().expect("Erreur lors de la fermeture du thread printer_thread");


}


//********************************************************************************************************************************
// Fonction tcp connection --- sert à mesurer le débit moyen et la perte de packets. 
fn tcp_connection(dist_addr: Ipv4Addr, port: u16, run_tcp: Arc<AtomicBool>, count_tcp: Arc<AtomicU16>){
    let distant_socket: SocketAddr = SocketAddr::new(IpAddr::V4(dist_addr), port);
    let mut stream: TcpStream = TcpStream::connect(distant_socket).unwrap();
    let mut array_vec: ArrayVec<u8, 8096> = ArrayVec::new();
        for _ in 0..array_vec.capacity() {
            array_vec.push(rand::random()); //INIT THE FUTURE WRITE BUFFER WITH FULL RANDOM VALUES (here packet size will be 32768 Bytes so 32kiB)
        }
    let write_buffer: [u8; 8096] = array_vec.into_inner().unwrap();
    let mut local_counter: u16 = 0;
    let mut total_packets: u64 = 0;
    let start: Instant = Instant::now();
    let mut partial_total_packets: u64 = 0;
    let mut partial_start: Instant = Instant::now();
    let mut buf: [u8; 8096] = [0; 8096];

    while run_tcp.load(Ordering::SeqCst) { //MAIN LOOP OF THE THREAD
        total_packets += 1; partial_total_packets += 1;
        let write_buffer_clone: [u8; 8096] = write_buffer.clone();
        stream.write(&write_buffer_clone).expect("Error while transmitting data from TCP socket.");

        
        let tmp_counter: u16 = count_tcp.load(Ordering::SeqCst);
        if tmp_counter!=local_counter {
            local_counter = tmp_counter.clone();
            stream.write("update".as_bytes()).expect("Error while transmitting update call");
            stream.read(&mut buf).unwrap();
            let partial_time: u128 = partial_start.elapsed().as_millis();
            let partial_speed: u64 = partial_total_packets*32768 / partial_time as u64;
            let partial_receiver_count: u64 = String::from_utf8(buf.to_vec()).unwrap().trim_end_matches('\0').parse().unwrap();
            let partial_drop_ratio: f64 = ((partial_total_packets as f64) - (partial_receiver_count as f64)) / (partial_total_packets as f64);
            println!(
                "Partial average speed : {} ({}bytes/{}s)\
                \nPartial packet drop ratio : {} ({} receiver count/{})", 
                partial_speed, 
                partial_total_packets*32768, 
                (partial_time as f64/1000 as f64), 
                partial_drop_ratio, 
                partial_receiver_count, 
                partial_total_packets
            );
            partial_total_packets = 0;
            partial_start = Instant::now();
        }
    }

    stream.write("finish".as_bytes()).expect("Error while transmitting finish call");
    let received = String::from_utf8(buf.to_vec()).unwrap();
    println!("received bytes: {}", received);           
    let receiver_count: u64 = String::from_utf8(buf.to_vec()).unwrap().trim_end_matches('\0').parse().unwrap();
    //LAST PRINT BEFORE CLOSING
    let total_time:u64 = start.elapsed().as_secs();
    let total_speed: u64 =total_packets*32768 / total_time;
    let drop_ratio: f64 = ((total_packets as f64) - (receiver_count as f64)) / (total_packets as f64);
 
    println!(
        "total average speed : {} ({} bytes/{} secs)\
        \nTotal packet drop ratio : {} ({} receiver count/{} total)", 
        total_speed, 
        total_packets*32768, 
        total_time, 
        drop_ratio, 
        receiver_count, 
        total_packets
    );

}



fn printer(run_print: Arc<AtomicBool>, count_print: Arc<AtomicU16>){
    let mut local_counter = count_print.load(Ordering::SeqCst);
    let mut time: Instant = Instant::now();
    while run_print.load(Ordering::SeqCst) {
        if time.elapsed().as_millis()>4800{
            local_counter += 1;
            println!("print counter : {}", local_counter);
            count_print.store(local_counter, Ordering::SeqCst);
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
