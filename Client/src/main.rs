use std::net::UdpSocket;

fn main() 
{
    let socket = UdpSocket::bind("localhost:8081").expect("couldn't bind to address");

    // start timer
    let start = std::time::Instant::now();
    
    let data = "message";

    // send data to socket address localhost:8080
    socket.send_to(data.as_bytes(), "localhost:8080").expect("error sending");

    let mut buf = [0; 17];
    let (amt, _src) = socket.recv_from(&mut buf).expect("error receiving");


    println!("Data: {}", String::from_utf8_lossy(&buf[..amt]));

}