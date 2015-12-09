extern crate rand;

use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use std::io::BufWriter;
use std::fs::File;

fn main() {
    let f = File::create("socket_producer_out.txt")
            .ok()
            .expect("Failed to create file.");
    let mut fw = BufWriter::new(f); 

    // fill array with random bytes
    let r = Range::new(0i32, 9);
    let mut rng = rand::thread_rng();
    let mut rand_data = String::new();
    for _ in 0..100 {
        let val = r.ind_sample(&mut rng)
            .to_string();
        rand_data.push_str(&val);
    }
    
    {
        let mut stream = TcpStream::connect("127.0.0.1:9090")
            .ok()
            .expect("Connection refused.");
        fw.write(rand_data.as_bytes())
            .ok()
            .expect("Failed to write to file.");

        // write 10 bytes of the string to the stream
        for out_buf in rand_data.into_bytes().chunks(10) {
            stream.write(&out_buf)
                .ok()
                .expect("Failed to write to stream.");
            let in_buf = &mut [0u8; 1];
            while in_buf[0] < 10 {
                stream.read(&mut in_buf[..])
                    .ok()
                    .expect("Failed to read from stream.");
                print!("in={} ", in_buf[0]);
            }
        }
        stream.shutdown(Shutdown::Both)
            .ok()
            .expect("Failed to close streams.");
    }
}
