use std::io;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let in_buf = &mut [0; 100]; 
    let f = File::create("pipe_consumer_in.txt")
            .ok()
            .expect("Failed to create file.");
    let mut writer = BufWriter::new(f);
    {
        io::stdin().read(&mut in_buf[..]).unwrap();
        writer.write(&in_buf[..])
                .ok()
                .expect("Failed to write to file.");
    }
}
