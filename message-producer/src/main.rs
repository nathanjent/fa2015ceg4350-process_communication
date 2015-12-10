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

        let r = Range::new(0i32, 9);
        let mut rng = rand::thread_rng();
        let child_tx = tx.clone();
        for _ in 0..100 {
            let x = r.ind_sample(&mut rng);
            child_tx.send(x as u8).unwrap();
            fpw.write_fmt(format_args!("{}", x))
            .ok()
            .expect("Failed to write to file.");
        }
        
    });

    // read channel
    let rx_buf = &mut [0i32; 100];
    for item in rx_buf.iter_mut() {
        *item = rx.recv().unwrap() as i32;
    }
    for x in rx_buf.iter() {
        fcw.write_fmt(format_args!("{}", x))
            .ok()
            .expect("Failed to write to file.");
    }
}
