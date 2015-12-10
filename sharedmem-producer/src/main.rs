extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::sync::Semaphore;

fn main() {
    // create a semaphore with 1 resource
    let sem = Semaphore::new(1);
    let buf_size = 10;
    let count = 0;
    let ring_buf = &mut [0i32; 10];
    let recv_total = 0;

    thread::spawn(move || {
        let fp = File::create("sharedmem_producer_out.txt")
            .ok()
            .expect("Failed to create file.");
        let mut fwp = BufWriter::new(fp);
        let r = Range::new(0i32, 9);
        let mut rng = rand::thread_rng();
        let in_pos = 0;

        loop {
            sem.acquire();

            {
                let _guard = sem.access();
                let rand = r.ind_sample(&mut rng);
                ring_buf[in_pos] = rand.clone();
                in_pos = (in_pos + 1) % buf_size;
                count += 1;
                fwp.write_fmt(format_args!("{}", rand))
                    .ok()
                    .expect("Failed to write to file.");
            }
            sem.release();
        } // guard released here
    });

    let fc = File::create("sharedmem_consumer_in.txt")
        .ok()
        .expect("Failed to create file.");
    let mut fwc = BufWriter::new(fc);
    let out_pos = 0;

    loop {
        sem.acquire();

        {
            let _guard = sem.access();
            let out = ring_buf[out_pos];
            fwc.write_fmt(format_args!("{}", out))
                .ok()
                .expect("Failed to write to file.");
            out_pos = (out_pos + 1) % buf_size;
            count -= 1;
        } // guard released here
        sem.release();
        recv_total += 1;
        if recv_total == 99 { break; }
    }
}
