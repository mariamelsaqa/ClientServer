use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() 
{
    let socket = UdpSocket::bind("localhost:8081").expect("Client couldn't bind to address");

    // start timer
    let start = std::time::Instant::now();
    
    let data = "message";

    let handle = thread::spawn(move || {
        for i in 1..10 {

            println!("hi number {} from the spawned thread!", i);
            //send data to socket address localhost:8080
            socket.send_to(data.as_bytes(), "localhost:8082").expect("Client error sending");

            let mut buf = [0; 17];

            let (amt, _src) = socket.recv_from(&mut buf).expect("Client error receiving");
            println!("Client Data {} from {}", String::from_utf8_lossy(&buf[..amt]), _src);
        }
    });
    handle.join().unwrap();
    


}
