use chrono::{Utc, DateTime};
use std::{fs::File, io::Read};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub message_size: usize,
}

impl Config {
    pub fn new(host: String, message_size: usize) -> Self { Self { host, message_size } }
}

pub fn get_config() -> Result<Config, std::io::Error> {
    let mut file = File::open("..//lib//src//Config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = serde_json::from_str(&contents)?;
    Ok(config)
}




pub struct Message {
    pub text : String,
    pub sender_address : String,
    pub reciever_address : String,
    pub time_sending : DateTime<Utc>,
}

impl Message {
    pub fn new(sender_address : String, reciever_address : String, text : String, time_sending : DateTime<Utc>) -> Message{
        Message {
            text : text,
            sender_address : sender_address,
            reciever_address : reciever_address,
            time_sending : time_sending,
        }
    }
}

pub struct User {

}


pub struct Server{

}
