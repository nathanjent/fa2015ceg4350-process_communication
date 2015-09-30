extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::process::{Command, Stdio};
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let f = File::create("producer_out.txt")
            .ok()
            .expect("Failed to create file.");
    let r = Range::new(1, 1001);
    let mut rng = rand::thread_rng();
    {
        let mut writer = BufWriter::new(f);
        let mut child = Command::new("pipe-consumer")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        let child_in = child.stdin.as_mut().unwrap();
        let mut x;
        for  i in (0..100).enumerate() {
            x = r.ind_sample(&mut rng);
            child_in.write(&[(x as u8)])
                    .ok()
                    .expect("Failed to write to child process.");
            writer.write(&[(x as u8)])
                    .ok()
                    .expect("Failed to write to file.");
        }
    }
}
