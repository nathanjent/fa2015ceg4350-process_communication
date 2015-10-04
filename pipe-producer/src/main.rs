extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::process::{Command, Stdio};
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let buffer_size = 10;
    let key = "COUNTER";
    env::set_var(key,  0);
    let f = File::create("producer_out.txt")
            .ok()
            .expect("Failed to create file.");
    let r = Range::new(0, 100);
    let mut rng = rand::thread_rng();
    {
        let mut writer = BufWriter::new(f);
        let mut child = Command::new("pipe-consumer")
            .stdin(Stdio::inherit())
            .spawn()
            .unwrap_or_else(|e| 
                { panic!("failed to execute process: {}", e) });

        let child_in = child.stdin.as_mut().unwrap();
        let mut x;
        for  i in (0..100).enumerate() {
            loop {
                match env::var(key) {
                    
                    Ok(val) => let mut counter = val),
                    Err(e) => println!("couldn't interpret {}: {}", key, e),
                }
                if counter < buffer_size {
                    x = r.ind_sample(&mut rng);
                    child_in.write_fmt(format_args!("{:02} ", x))
                            .ok()
                            .expect("Failed to write to child process.");
                    writer.write_fmt(format_args!("{:02} ", x))
                            .ok()
                            .expect("Failed to write to file.");
                    counter = counter + 1;
                    env::set_var(key, "VALUE");
                    break;
                }
            }
        }
    }
}
