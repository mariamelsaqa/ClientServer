use std::net::UdpSocket;
use std::{thread, time::Duration};
fn main() {
     // create a socket
     let socket = UdpSocket::bind("localhost:8084").expect("Server Could not bind socket");

     // create a buffer
     let mut buf = [0; 19];
     println!("Listening");
     // read data from socket
    loop
    {
       
        let (amt, src) = socket.recv_from(&mut buf).expect("error reading");
        let sock = socket.try_clone().expect("Failed to clone socket");
        let handle =thread::spawn(move||  {
            println!("Server Received {} bytes from {}", amt, src);
            println!("Server Data: {}", String::from_utf8_lossy(&buf[..amt]));

            // write data to socket
            let data = "ping acknowledged";
            sock.send_to(data.as_bytes(), src).expect("error writing");
            println!("Server Sent {} bytes to {}", data.len(), src);
        });
      
        handle.join().unwrap();
    }
}
