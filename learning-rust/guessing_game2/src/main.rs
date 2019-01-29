use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let num = rand::thread_rng().gen_range(1, 101);
    println!("{}", num);
}
