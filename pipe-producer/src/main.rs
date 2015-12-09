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
    let r = Range::new(0i32, 9);
    let mut rng = rand::thread_rng();
    let out_buf = &mut [0i32; 100];
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
        for out in out_buf.iter() {
            child_in.write_fmt(format_args!("{}", out))
                .ok()
                .expect("Failed to write to child process.");
            writer.write_fmt(format_args!("{}", out))
                .ok()
                .expect("Failed to write to file.");
        }
    }
}
