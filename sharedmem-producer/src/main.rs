extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread;


fn main() {
    let buf: Vec<_> = (0..100u32).collect();
    let shared_buf = Arc::new(buf);

    let fp = File::create("sharedmem_producer_out.txt")
        .ok()
        .expect("Failed to create file.");
    let mut fwp = BufWriter::new(fp);
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    let out_buf = &mut [0u8; 100];
    for item in out_buf.iter_mut() {
        *item = r.ind_sample(&mut rng);
    }

    thread::spawn(move || {
        
    });
    {
        fwp.write(&mut out_buf[..])
            .ok()
            .expect("Failed to write to file.");
    }
    let in_buf = &mut [0; 100]; 
    let fc = File::create("sharedmem_consumer_in.txt")
        .ok()
        .expect("Failed to create file.");
    let mut fwc = BufWriter::new(fc);
    {
        fwc.write(&in_buf[..])
            .ok()
            .expect("Failed to write to file.");
    }
}
