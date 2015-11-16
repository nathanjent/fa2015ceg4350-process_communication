use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io;
use std::io::LineWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut data = String::new(); 
    let file = File::create("consumer_in.txt")
            .ok()
            .expect("Failed to create file.");
    let mut file = LineWriter::new(file);

    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();

// accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, file);
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}

fn handle_client(stream: TcpStream, file: LineWriter<File>) {
    let output= stream.write(&[1]);
    let input = stream.read(&mut [0; 128]);
    let mut data = String::new(); 
    
    stream.read_to_string(&mut data).unwrap();
    file.write(data.as_bytes())
                .ok()
                .expect("Failed to write to file.");

}
