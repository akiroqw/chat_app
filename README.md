## Introduction

A small console application for user chat. Description of receiving and sending messages to the server.

![Introduction](https://i.pinimg.com/originals/05/f1/7d/05f17d6e87ad18f65940f896f4cf11a4.gif)

## Requirement
* Rust version 1.68.2 must be installed (during development we started with version 1.68.2, so we cannot guarantee that earlier versions will work);

## Installation
* Create a folder and open it through the console, or create a project and open the console through the IDE
* Clone repository
```
$ git clone https://github.com/akiroqw/chat_app.git
```

## Usage
* If required, the following parameters must be changed(chat_server/src/main.rs/ or chat_client/src/main.rs):
```rs
const LOCAL : &str = "127.0.0.1:6000";
const MESSAGE_SIZE : usize = 32;
```
* This is the required message size and the host where TCPListener listens for connections and TCPStream which opens streams and connects to the host. 
You can change the listening host, but you also need to change the connection parameters.

## Сhat system
The chat system is implemented by using standard libraries and classes TCPListener and TCPStream.
The server is implemented locally. Also the host thread is not blocked and does not wait for connection, which is implemented by this line:
```rs
set_nonblocking(true).expect("Failed to initalize non-blocking.");
```
The logic of the application is quite simple. Which increases understanding and usability.

