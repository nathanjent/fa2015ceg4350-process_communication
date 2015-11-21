use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::LineWriter;
use std::io::prelude::*;
use std::fs::File;

fn handle_client(mut stream: TcpStream) {
    //let _ = stream.write(&[1]);

    //let _ = stream.read(&mut [0; 128]);
    let mut data = String::new(); 
    let file = File::create("consumer_in.txt")
            .ok()
            .expect("Failed to create file.");
    let mut file = LineWriter::new(file); 
    
    stream.read_to_string(&mut data)
        .ok()
        .expect("Failed to read socket.");
    file.write(data.as_bytes())
            .ok()
            .expect("Failed to write to file.");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();

// accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream);
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}

