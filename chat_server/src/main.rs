use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use chrono::{DateTime, Utc};

fn main() {

    const LOCAL : &str = "127.0.0.1:6000";
    const MESSAGE_SIZE : usize = 32;

    let server : TcpListener = TcpListener::bind(LOCAL).expect("Failed to connect to the server!");
    server.set_nonblocking(true).expect("Failed to initalize non-blocking");
    



    println!("The server is running: [{}]", Utc::now());
}
