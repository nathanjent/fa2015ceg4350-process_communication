extern crate rand;

use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use std::io::BufWriter;
use std::fs::File;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let fc = File::create("message_consumer_out.txt")
        .ok()
        .expect("Failed to create file.");
    let mut fcw = BufWriter::new(fc);
    let (tx, rx) = channel();
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    let tx_buf = [r.ind_sample(&mut rng); 100];
    thread::spawn(move || {
        let fp = File::create("message_producer_out.txt")
            .ok()
            .expect("Failed to create file.");
        let mut fpw = BufWriter::new(fp); 
        fpw.write(&tx_buf[..])
            .ok()
            .expect("Failed to write to file.");
        for x in tx_buf.iter() {
            tx.send(x).unwrap();
        }
    });
    let rx_buf = &mut [0u8; 100];
    for item in rx_buf.iter_mut() {
        *item = *rx.recv().unwrap();
    }
    fcw.write(&mut rx_buf[..])
        .ok()
        .expect("Failed to write to file.");
}
