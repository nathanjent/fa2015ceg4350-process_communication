extern crate rand;

use std::net::TcpListener;
use std::net::{SocketAddr, Ipv4Addr};
use std::str;
use std::cell::Cell;
use rand::distributions::{IndependentSample, Range};
use std::io::{BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::thread;

fn main() {
    let f = File::create("producer_out.txt")
            .ok()
            .expect("Failed to create file.");
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    {
        let mut writer = BufWriter::new(f); 
        let addr = SocketAddr {
            ip: Ipv4Addr::new(127, 0, 0, 1),
            port: 9090
        };
        let mut x;
        let mut acceptor = TcpListener::bind(addr).listen().unwrap();

        println!("Listener is ready");

        let mut outputBuffer = [0,..100];
        // only retrieve 100 items
        for  i in (0..100).enumerate() {
            x = r.ind_sample(&mut rng);
            writer.write_fmt(format_args!("{:02} ", x))
                .ok()
                .expect("Failed to write to file.");
        }

        loop {
            let stream = acceptor.accept().unwrap();
            let stream = Cell::new(stream);
            thread::spawn(move || {
                let mut buf = [0,..512];
                let mut stream = stream.take();
                loop {
                    match stream.read(buf) {

                        Some(count) => {
                            println!("read {} bytes: {}",
                                    count.to_str(),
                                    str::from_utf8(buf)
                               );
                             stream.write(b"Hello World\r\n");
                             break; // close connection after read
                        }
                        None => { println!("Finished"); break } // connection is closed by client
                    }
                }
            });
        }
    }
}

