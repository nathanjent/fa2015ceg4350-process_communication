use std::io;

fn main() {
    let mut data = String::new();
    loop {
        io::stdin().read_line(&mut data).unwrap();
        println!("{}",data);
    }
}