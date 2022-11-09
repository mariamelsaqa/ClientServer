use std::net::UdpSocket;
use local_ip_address::local_ip;

fn main() {
    let mut my_local_ip = local_ip().unwrap().to_string();
    server1 = my_local_ip.push_str("8083"); //server 1 port
    server2 = my_local_ip.push_str("8084"); //server 2 port
    server3 = my_local_ip.push_str("8085"); //server 3 port
    my_local_ip.push_str(":8082"); //receive requests
    let socket_s1 = UdpSocket::bind(server1).expect("couldn't bind to address");
    let socket_s2 = UdpSocket::bind(server2).expect("couldn't bind to address");
    let socket_s3 = UdpSocket::bind(server3).expect("couldn't bind to address");

    let socket_client = UdpSocket::bind(my_local_ip).expect("couldn't bind to address");
    let mut buf1 = [0; 17];
    let mut buf2 = [0; 17];
    let mut buf3 = [0; 17];
    let mut buf4 = [0; 17];
    let (amt, _src) = socket_client.recv_from(&mut buf1).expect("couldn't receive data");
    let (received_server1,_src1) = server1.recv_from(&mut buf2).expect("error2");
    let (received_server2,_src2) = server2.recv_from(&mut buf3).expect("error2");
    let (received_server3,_src3) = server3.recv_from(&mut buf4).expect("error2");
    if(String::from_utf8_lossy(&buf2[..received_server1]) == true)
        {
            socket_client.send_to(String::from_utf8_lossy(&buf1[..amt]).as_bytes(), _src1).expect("couldn't send data");
        }
    else if (String::from_utf8_lossy(&buf3[..received_server2]) == true)
        {
            socket_client.send_to(String::from_utf8_lossy(&buf1[..amt]).as_bytes(), _src2).expect("couldn't send data");
        }
    else
        {
            socket_client.send_to(String::from_utf8_lossy(&buf1[..amt]).as_bytes(), _src3).expect("couldn't send data");
        }   
}   
