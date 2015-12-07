extern crate rand;

use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use std::io::BufWriter;
use std::fs::File;
use std::thread;
use std::sync::mpsc::sync_channel;

fn main() {
    let fc = File::create("message_consumer_out.txt")
        .ok()
        .expect("Failed to create file.");
    let mut fcw = BufWriter::new(fc);
    
    // sync channel blocks at 10 until received
    let (tx, rx) = sync_channel::<u8>(10);

    // spawn separate process
    thread::spawn(move || {
        let fp = File::create("message_producer_out.txt")
            .ok()
            .expect("Failed to create file.");
        let mut fpw = BufWriter::new(fp); 

        let r = Range::new(0, 100);
        let mut rng = rand::thread_rng();
        let tx_buf = &mut [0u8; 100];
        let child_tx = tx.clone();
        for item in tx_buf.iter_mut() {
            let x = r.ind_sample(&mut rng);
            child_tx.send(x).unwrap();
            *item = x;
        }
        fpw.write(&tx_buf[..])
            .ok()
            .expect("Failed to write to file.");
    });

    // read channel
    let rx_buf = &mut [0u8; 100];
    for item in rx_buf.iter_mut() {
        *item = rx.recv().unwrap();
    }
    fcw.write(&mut rx_buf[..])
        .ok()
        .expect("Failed to write to file.");
}
