use std::net::UdpSocket;

fn main() {
    loop
    {
        let socket_client = UdpSocket::bind("localhost:8082").expect("couldn't bind to address");
        let socket_server = UdpSocket::bind("localhost:8083").expect("couldn't bind to address");
        let mut buf1 = [0; 17];

        // receive from client 
        let (amt, src) = socket_client.recv_from(&mut buf1).expect("error reading");
        println!("Agent Received from client");

        
        //forward request to server
        socket_server.send_to(&buf1[..amt],"localhost:8080").expect("error sending");
        let (amt, _src) = socket_server.recv_from(&mut buf1).expect("error receiving");
        println!("Data: {} from: {}", String::from_utf8_lossy(&buf1[..amt]), _src);

        //forward response to client
        socket_client.send_to(&buf1[..amt],"localhost:8081").expect("error sending");
        println!("Agent Sent to client");
    }
}