extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::process::{Command, Stdio};
use std::io::{BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let f = File::create("pipe_producer_out.txt")
        .ok()
        .expect("Failed to create file.");
    let mut writer = BufWriter::new(f);
    let r = Range::new(0x21, 0x7E);
    let mut rng = rand::thread_rng();
    let out_buf = &mut [0u8; 100];
    for item in out_buf.iter_mut() {
        *item = r.ind_sample(&mut rng);
    }
    {
        let mut child = Command::new("pipe-consumer")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| 
                { panic!("failed to execute process: {}", e) });

        let child_in = child.stdin.as_mut().unwrap();
        child_in.write(&mut out_buf[..])
            .ok()
            .expect("Failed to write to child process.");
        writer.write(&mut out_buf[..])
            .ok()
            .expect("Failed to write to file.");
    }
}
