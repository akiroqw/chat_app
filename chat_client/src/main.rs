use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use lib::*;

fn main() {

    let config = get_config().unwrap();

    let mut client : TcpStream = TcpStream::connect(config.host).expect("Stream failed to connect!");
    client.set_nonblocking(true).expect("Failed to initalize non-blocking.");

    let (tx, rx) = mpsc::channel::<String>();


    thread::spawn(move || loop {
        let mut buff = vec![0; config.message_size];
        
        match client.read_exact(&mut buff) {

            Ok(_) =>{
                //let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) =>{
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) =>{
                let mut buff  = msg.clone().into_bytes();
                buff.resize(config.message_size, 0);
                client.write_all(&buff).expect("Writing to socket failed!");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

   
    loop {

        println!("Write a Message --> ");

        let mut buff = String::new();
        io::stdin().read_line(& mut buff).expect("Reading Failed!!!");

        let msg = buff.trim().to_string();

        if msg == "!quit" || tx.send(msg).is_err() {break;}

    }

}
