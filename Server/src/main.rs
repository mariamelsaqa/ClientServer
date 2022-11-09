use std::net::UdpSocket;
fn main() {
    loop
    {
        // create a socket
        let socket = UdpSocket::bind("localhost:8080").expect("Could not bind socket");

        // create a buffer
        let mut buf = [0; 19];

        // read data from socket
        let (amt, src) = socket.recv_from(&mut buf).expect("error reading");

        // print data
        println!("Received {} bytes from {}", amt, src);
        println!("Data: {}", String::from_utf8_lossy(&buf[..amt]));

        // write data to socket
        let data = "ping acknowledged";
        socket.send_to(data.as_bytes(), src).expect("error writing");
    }
}
