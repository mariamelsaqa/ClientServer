use std::cmp;
use std::net::UdpSocket;
use std::{thread,time::Duration};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
fn main() {
    let mut destination_add1 = "10.7.57.71:8086";
    let mut destination_add2 = "10.7.57.2:8087";
    let mut destination_add3 = "10.7.57.93:8088";
    let mut s1 = true;
    let mut s2 = false;
    let mut s3 = true;
    let mut current_address = destination_add3;
    let socket = UdpSocket::bind("10.7.57.128:8082").expect("Server Could not bind socket");
    let socket2 = UdpSocket::bind ("10.7.57.128:8083").expect("server Could not bind socket");
    let socket_bool1 = UdpSocket::bind ("10.7.57.128:8086").expect("server Could not bind socket");
    let socket_bool2 = UdpSocket::bind ("10.7.57.128:8087").expect("server Could not bind socket");
    let socket_bool3 = UdpSocket::bind ("10.7.57.128:8088").expect("server Could not bind socket");
     // create a buffer
     let mut buf = [0; 19];
     // read data from socket
     let mut buf1 = [0; 19];
     let mut buf2 = [0; 19];
     let mut buf3 = [0; 19];
     // read data from socket
     let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
     let (tx2, rx2): (Sender<String>, Receiver<String>) = mpsc::channel();
     let (tx3, rx3): (Sender<String>, Receiver<String>) = mpsc::channel();
    
    let thread_tx = tx.clone();
    let thread_tx2 = tx2.clone();
    let thread_tx3 = tx3.clone();
    let handle1 = thread::spawn(move || {
        loop{
            let (amt, src) = socket_bool1.recv_from(&mut buf1).expect("error reading");
            thread_tx.send(String::from_utf8_lossy(&buf1[..amt]).to_string());
        }
    });
    let handle2 = thread::spawn(move || {
        loop{
            let (amt, src) = socket_bool2.recv_from(&mut buf2).expect("error reading");
            thread_tx2.send(String::from_utf8_lossy(&buf2[..amt]).to_string());
        }
    });
    let handle3= thread::spawn(move || {
        loop{
            let (amt, src) = socket_bool3.recv_from(&mut buf3).expect("error reading");
            thread_tx3.send(String::from_utf8_lossy(&buf3[..amt]).to_string());
        }
    });
    let mut b1 = 1;
    let mut b2 = 1;
    let mut b3 = 1;
    loop
    {
        println!("Listening");
        let (amt, src) = socket.recv_from(&mut buf).expect("error reading");
        println!("Listened");
        let sock = socket.try_clone().expect("Failed to clone socket");
        let sock2 = socket2.try_clone().expect("failed to clone socket");

        match rx.try_recv(){
            Ok(result) => b1 = result.parse::<i32>().unwrap(),
            Err(e) => println!(""),
        }
        match rx2.try_recv(){
            Ok(result2) => b2 = result2.parse::<i32>().unwrap(),
            Err(e) => println!(""),
        }
        match rx3.try_recv(){
            Ok(result3) =>  b3 = result3.parse::<i32>().unwrap(),
            Err(e) => println!(""),
        }


        if(b1 == -1)
            {s1 = false;}
        else {s1 = true;}

        if(b2 == -1)
            {s2 = false;}
        else {s2 = true;}

        if(b3 == -1)
            {s3 = false;}
        else {s3 = true;}

        if (!s1 && !s2 && !s3)
        {
            s1 = true;
            s2 = true;
            s3 = true;
        }

        let sock = socket.try_clone().expect("Failed to clone socket");
        let sock2 = socket2.try_clone().expect("failed to clone socket");
        loop{
            if current_address == destination_add3
                {current_address = destination_add1;}
            else if current_address == destination_add1
                {current_address = destination_add2;}
            else
                {current_address = destination_add3;}
            if (current_address == destination_add3 && s3 == true)
                {break;}
            else if (current_address == destination_add1 && s1 == true)
                {break;}
            else if (current_address == destination_add2 && s2 == true)
                {break;}
            } 
        let handle =thread::spawn(move||  {
            sock2.send_to(String::from_utf8_lossy(&buf[..amt]).as_bytes(),current_address).expect("error writing");
            let mut buf2 = [0; 19];
            let (amt2, src2) = sock2.recv_from(&mut buf2).expect("error reading");  
            sock.send_to(&buf[..amt2],"10.7.57.128:8080").expect("error sending msg");
        });
        println!("sent");
        handle.join().unwrap();
    }
}