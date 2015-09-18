extern crate rand;

use rand::Rng;
use std::process::{Command, Stdio};
use std::io::{Write};

fn main() {
    let mut child = Command::new("pipe-consumer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let child_in = child.stdin.as_mut().unwrap();
    for  i in (0..100).enumerate() {
        let mut x = rand::thread_rng().gen_range(1, 101);
        child_in.write(&mut x.asByte()).unwrap();
    }
}
