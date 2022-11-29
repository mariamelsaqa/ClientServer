use std::net::UdpSocket;
use std::{thread, time::Duration};
fn main() {
    let mut destination_add1 = "localhost:8084";
    let mut destination_add2 = "localhost:8085";
    let mut destination_add3 = "localhost:8086";
    let mut current_address = destination_add3;
    let socket = UdpSocket::bind("localhost:8082").expect("Server Could not bind socket");

     // create a buffer
     let mut buf = [0; 19];
     // read data from socket
    loop
    {
        println!("Listening");
        let (amt, src) = socket.recv_from(&mut buf).expect("error reading");
        println!("Listened");
        let sock = socket.try_clone().expect("Failed to clone socket");
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
            sock.send_to(String::from_utf8_lossy(&buf[..amt]).as_bytes(),current_address).expect("error writing");
            
        });
        println!("sent");
        handle.join().unwrap();
    }
}