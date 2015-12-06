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
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    let out_buf = &mut [0u8; 100];
    for item in out_buf.iter_mut() {
        *item = r.ind_sample(&mut rng);
    }
    {
        let mut stream = TcpStream::connect("127.0.0.1:9090")
            .ok()
            .expect("Connection refused.");
        for items in out_buf.take(10) {
            fw.write(&items)
                .ok()
                .expect("Failed to write to file.");
            stream.write(&items)
                .ok()
                .expect("Failed to write to stream.");
        }
        stream.shutdown(Shutdown::Both)
            .ok()
            .expect("Failed to close streams.");
    }
}
