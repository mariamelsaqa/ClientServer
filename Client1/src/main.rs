use std::net::UdpSocket;
use std::{thread, time::Duration};
use std::time::{Instant};
fn main() 
{
    let socket = UdpSocket::bind("10.7.57.128:8080").expect("Client couldn't bind to address");

    // start timer
    let mut total_time = Duration::ZERO;
    let mut  handle = thread::spawn( || {});
    for i in 1..200000 {
        let mut data = i.to_string();
        let client_name = "-client 1".to_string();
        data.push_str(&client_name);

        let sock = socket.try_clone().expect("Failed to clone socket");
        thread::sleep_ms(100);
        handle = thread::spawn(move || {
        

        println!("hi number {} from the spawned thread!", i);
        
        //send data to socket address localhost:8080
        //thread::sleep(time::duration::from_millis(1000));
        let now = Instant::now();
        sock.send_to(data.as_bytes(), "10.7.57.128:8082").expect("Client error sending");
        let mut buf = [0; 17];

        let (amt, _src) = sock.recv_from(&mut buf).expect("Client error receiving");
        let elapsed_time = now.elapsed();
        println!("Client Data {} from {}", String::from_utf8_lossy(&buf[..amt]), _src);
        total_time = total_time + elapsed_time;
        let mut avg = total_time / (i+1);
        println! ("avg = {:?}", avg);
            
        });
        //thread::sleep(Duration::from_millis(4000));
    }
    handle.join().unwrap();
    
    


}