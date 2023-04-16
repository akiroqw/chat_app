use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use chrono::{DateTime, Utc};
use std::time::Duration;

#[allow(dead_code)]
struct Client {
    name: String,
    id: u64,
}

#[allow(dead_code)]
impl Client {
    fn new(name: String, id: u64) -> Self {
        Self {
            name,
            id,
        }
    }
}

fn main() {

    const LOCAL : &str = "127.0.0.1:6000";
    const MESSAGE_SIZE : usize = 32;

    let server : TcpListener = TcpListener::bind(LOCAL).expect("Failed to connect to the server!");
    server.set_nonblocking(true).expect("Failed to initalize non-blocking.");

    println!("The server is running: [{}]", Utc::now());
    
    let mut clients =  vec![];

    let (tx, rx) = mpsc::channel::<String>();


    loop{

        if let Ok((mut socket, address)) = server.accept(){

            println!("Client {} connected.", address);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client!"));


            thread::spawn(move || loop {

                let mut buff = vec![0; MESSAGE_SIZE];
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

                buff.resize(MESSAGE_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()

            }).collect::<Vec<_>>();
        }

        thread::sleep(Duration::from_millis(100));
        
    }

}
