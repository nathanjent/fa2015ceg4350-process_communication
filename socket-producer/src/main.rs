extern crate rand;

use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use std::io::BufWriter;
use std::fs::File;

fn main() {
    let buf_size = 10;
    let f = File::create("socket_producer_out.txt")
            .ok()
            .expect("Failed to create file.");
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    {
        let mut fw = BufWriter::new(f); 
        let mut out;
        let mut in_buf = [0; 10];
        let mut out_buf = [0];
        let mut item = 0;
        let mut count = 0;
        let mut stream = TcpStream::connect("127.0.0.1:9090")
            .ok()
            .expect("Connection refused.");
        loop {
            while item < buf_size {
                out = r.ind_sample(&mut rng);
                out_buf.
                fw.write(&mut out_buf[..])
                    .ok()
                    .expect("Failed to write to file.");
                stream.write_fmt(format_args!("{:02} ", out))
                    .ok()
                    .expect("Failed to write to stream.");
                item = item + 1;
                count= count + 1;
                if count == 100 { break; }
            }
            match stream.read(&mut in_buf[..]) {
                Ok(n) => {
                    item = item - n;
                }
                Err(e) => { }
            }
            if count == 100 { break; }
        }
        stream.shutdown(Shutdown::Both)
            .ok()
            .expect("Failed to close streams.");
    }
}
