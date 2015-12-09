use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

fn handle_client(mut stream: TcpStream) {
    let f = File::create("socket_consumer_in.txt")
            .ok()
            .expect("Failed to create file.");
    let mut fw = BufWriter::new(f); 
    loop {
        let mut count = 0;
        let mut in_buf = [0u8; 10];
        while count < 10 {
            match stream.read(&mut in_buf) {
                Ok(read_size) => {
                    count += read_size as u8;
                    print!("in={} ", count);
                }
                Err(e) => {
                    print!("Failed to read socket. {}", e); 
                }
            }
            let count_buf = [count];
            stream.write(&count_buf[..])
                .ok()
                .expect("Failed to write to socket.");
            if count > 0 {
                fw.write(&mut in_buf[..])
                    .ok()
                    .expect("Failed to write to file.");
            }
        }
    }
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
            Err(e) => {print!("{}", e); /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}

