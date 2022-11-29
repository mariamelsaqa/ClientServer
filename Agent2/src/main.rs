use std::net::UdpSocket;
use std::{thread, time::Duration};
fn main() {
    let mut destination_add1 = "localhost:8086";
    let mut destination_add2 = "localhost:8087";
    let mut destination_add3 = "localhost:8088";
    let mut current_address = destination_add3;
    let socket = UdpSocket::bind("localhost:8084").expect("Server Could not bind socket");
    let socket2 = UdpSocket::bind ("localhost:8085").expect("server Could not bind socket");

     // create a buffer
     let mut buf = [0; 19];
     // read data from socket
    loop
    {
        println!("Listening");
        let (amt, src) = socket.recv_from(&mut buf).expect("error reading");
        println!("Listened");
        let sock = socket.try_clone().expect("Failed to clone socket");
        let sock2 = socket2.try_clone().expect("failed to clone socket");
        if current_address == destination_add1
        {
            current_address = destination_add2;
        }
        else if current_address == destination_add2 {
            current_address = destination_add3;
        }
        else{
            current_address = destination_add1;
        }
        let handle =thread::spawn(move||  {
            sock2.send_to(String::from_utf8_lossy(&buf[..amt]).as_bytes(),current_address).expect("error writing");
            let mut buf2 = [0; 19];
            let (amt2, src2) = sock2.recv_from(&mut buf2).expect("error reading");  
            sock.send_to(&buf[..amt2],"localhost:8081").expect("error sending msg");
            
        });
        println!("sent");
        handle.join().unwrap();
    }
}