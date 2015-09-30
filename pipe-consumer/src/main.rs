use std::io;
use std::io::LineWriter;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut data = String::new(); 
    let f = File::create("consumer_in.txt")
            .ok()
            .expect("Failed to create file.");
    let mut writer = LineWriter::new(f);
    
    for  i in (0..100).enumerate()  {
        io::stdin().read_line(&mut data).unwrap();
        writer.write(data.as_bytes())
                .ok()
                .expect("Failed to write to file.");
        
        println!("{}",data);
    }
}