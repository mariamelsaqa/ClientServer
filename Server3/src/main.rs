use std::cmp;
use std::net::UdpSocket;
use std::{thread,time::Duration};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use rand::Rng;
fn main() {
     // create a socket
     let handle_election  = thread::spawn(move || {distributed_election()});
     let socket = UdpSocket::bind("10.7.57.93:8088").expect("Server Could not bind socket");
     let mut agents = vec![];

     let mut requests_counter = 0;
     // create a buffer
     let mut buf = [0; 19];
     println!("Listening");
     // read data from socket
    loop
    {
       
        let (amt, src) = socket.recv_from(&mut buf).expect("error reading");
        requests_counter = requests_counter + 1;
        println!("Requests Counter: {:?}", requests_counter);
        if (agents.contains(&src) == false)
        {
            agents.push(src);
        }
        let sock = socket.try_clone().expect("Failed to clone socket");
        let handle =thread::spawn(move||  {
           // println!("Server Received {} bytes from {}", amt, src);
            println!("Server Data: {}", String::from_utf8_lossy(&buf[..amt]));

            // write data to socket
            let data = "ping acknowledged";
            sock.send_to(data.as_bytes(), src).expect("error writing");
            //println!("Server Sent {} bytes to {}", data.len(), src);
        });
      
        handle.join().unwrap();
    }
}


fn distributed_election()
{
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (tx2, rx2): (Sender<String>, Receiver<String>) = mpsc::channel();
    let socket = UdpSocket::bind("10.7.57.93:8093").expect ("server Could not bind socket");// server2
    let socket2 = UdpSocket::bind("10.7.57.93:8094").expect ("server Could not bind socket");// server3
    let socket3 = UdpSocket::bind("10.7.57.93:8095").expect ("server Could not bind socket");// agent
    let socket4 = UdpSocket::bind("10.7.57.93:8096").expect ("server Could not bind socket");// agentr3
    let mut rng = rand::thread_rng();
    let mut buf1 = [0;19];
    let mut buf2 = [0;19];
    thread::sleep_ms(10000);
    let mut amt2 = 4;
    let mut amt1 = 4;
    //let (amt1,src1) = NULL;
    let sock = socket.try_clone().expect("failed to clone socket");
    let sock2 = socket2.try_clone().expect("failed to clone socket");
    let thread_tx = tx.clone();
    let thread_tx2 = tx2.clone();
    let handle = thread::spawn(move || {
        loop {
        let (amt1,src1)= sock.recv_from(&mut buf1).expect("error reading");
        //println! ("received");
       // println!("{}",String::from_utf8_lossy(&buf1[..amt1]));
        thread_tx.send(String::from_utf8_lossy(&buf1[..amt1]).to_string()).unwrap();
         }
    });
    let handle2 = thread::spawn(move || {
         loop{
        let  (amt2,src2) = sock2.recv_from(&mut buf2).expect("error reading");
        //println! ("received");
      //  println!("{}",String::from_utf8_lossy(&buf2[..amt2]));
        thread_tx2.send(String::from_utf8_lossy(&buf2[..amt2]).to_string()).unwrap();
         }
    });

    let mut state = true;
    
    loop {
        thread::sleep_ms(60000);
        let mut mynum;
        if(state){
            mynum = rng.gen_range(0..100);
    
            
        }
            else{
                mynum = -1;
                println!("Iam down");
    
            }
            //println!("{}",mynum);


        socket.send_to(mynum.to_string().as_bytes(),"10.7.57.71:8090").expect("error writing"); //send server 1
        socket2.send_to(mynum.to_string().as_bytes(),"10.7.57.2:8092").expect("error writing"); //send server 2

       // println!("Sent");


       
        //println!("Created threads");
        let mut server1 = rx.recv().unwrap().parse::<i32>().unwrap();
        let mut server2 = rx2.recv().unwrap().parse::<i32>().unwrap();

         
       // println!("server 1 ={:?}",server1);
       // println!("server 2 ={:?}",server2);
       // println!("server 3 ={}",mynum);

        let max = cmp::max(cmp::max(mynum,server1),server2);
        
        if(max == mynum)
        {
            println!("max = {}",max);
            state = false;
            mynum = -1;

            socket3.send_to("-1".as_bytes(),"10.7.57.128:8088").expect("err");
            socket4.send_to("-1".as_bytes(),"10.7.57.88:8088").expect("err");


            socket.send_to(mynum.to_string().as_bytes(),"10.7.57.71:8090").expect("error writing"); //send server 1
            socket2.send_to(mynum.to_string().as_bytes(),"10.7.57.2:8092").expect("error writing"); //send server 2
            println!("server 3 down");
            thread::sleep_ms(15000);
            println!("server 3 up");

            socket3.send_to("1".as_bytes(),"10.7.57.128:8088").expect("err");
            socket4.send_to("1".as_bytes(),"10.7.57.88:8088").expect("err");

            state = true;
        }
        else
        {
            socket3.send_to("1".as_bytes(),"10.7.57.128:8088").expect("err");
            socket4.send_to("1".as_bytes(),"10.7.57.88:8088").expect("err");
        }

    }

}