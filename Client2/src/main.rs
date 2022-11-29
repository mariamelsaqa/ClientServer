use std::net::UdpSocket;
use std::{thread, time::Duration};
fn main() 
{
    let socket = UdpSocket::bind("localhost:8081").expect("Client couldn't bind to address");

    // start timer
    let start = std::time::Instant::now();
    
    let mut  handle = thread::spawn( || {});
    for i in 1..100 {
        let data = i.to_string();

        let sock = socket.try_clone().expect("Failed to clone socket");
        handle = thread::spawn(move || {
        

        println!("hi number {} from the spawned thread!", i);
        //send data to socket address localhost:8080
        sock.send_to(data.as_bytes(), "localhost:8083").expect("Client error sending");
        let mut buf = [0; 17];

        let (amt, _src) = sock.recv_from(&mut buf).expect("Client error receiving");
        println!("Client Data {} from {}", String::from_utf8_lossy(&buf[..amt]), _src);
            
        });
        //thread::sleep(Duration::from_millis(4000));
    }
    handle.join().unwrap();
}