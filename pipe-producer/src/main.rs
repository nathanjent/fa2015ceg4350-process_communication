extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::process::{Command, Stdio};
use std::io::{BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let f = File::create("producer_out.txt")
            .ok()
            .expect("Failed to create file.");
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    {
        let mut writer = BufWriter::new(f);
        let mut child = Command::new("./pipe-consumer")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| 
                { panic!("failed to execute process: {}", e) });

        let child_in = child.stdin.as_mut().unwrap();
        let mut x;
        
        // only retrieve 100 items
        for  i in (0..100).enumerate() {
            x = r.ind_sample(&mut rng);
            child_in.write_fmt(format_args!("{:02} ", x))
                    .ok()
                    .expect("Failed to write to child process.");
            writer.write_fmt(format_args!("{:02} ", x))
                    .ok()
                    .expect("Failed to write to file.");
        }
    }
}
