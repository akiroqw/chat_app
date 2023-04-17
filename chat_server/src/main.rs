use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use chrono::{Utc};
use std::time::Duration;
use lib::*;


fn main() -> ! {

    let config = get_config().unwrap();
    
    let server : TcpListener = TcpListener::bind(config.host).expect("Failed to connect to the server!");
    server.set_nonblocking(true).expect("Failed to initalize non-blocking.");

    println!("The server is running: [{}]", Utc::now());
    
    let mut clients: Vec<std::net::TcpStream> =  vec![];

    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel::<String>();

    loop{

        if let Ok((mut socket, address)) = server.accept(){

            println!("Client {} connected.", address);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client!"));
            

            thread::spawn(move || loop {

                let mut buff = vec![0; config.message_size];
                match socket.read_exact(& mut buff) {

                    Ok(_) =>{
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid convert to UTF8.");

                        println!("{} : {:?}", address, msg);
                        
                        tx.send(msg).expect("Failed to send message!");
                },

                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("Closing connection with: {}", address);
                    break;
                }
            }

            thread::sleep(Duration::from_millis(100));

            });
        }

        if let Ok(msg) = rx.try_recv(){

                clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();

                buff.resize(config.message_size, 0);

                client.write_all(&buff).map(|_| client).ok()

            }).collect::<Vec<_>>();
        }

        thread::sleep(Duration::from_millis(100));
    }

}
