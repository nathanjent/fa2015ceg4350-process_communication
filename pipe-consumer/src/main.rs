use std::io;
use std::io::LineWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut data = String::new(); 
    let f = File::create("pipe_consumer_in.txt")
            .ok()
            .expect("Failed to create file.");
    let mut writer = LineWriter::new(f);
    {
        io::stdin().read_line(&mut data).unwrap();
        writer.write(data.as_bytes())
                .ok()
                .expect("Failed to write to file.");
    }
}
