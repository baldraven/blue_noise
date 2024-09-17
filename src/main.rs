use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = &args[1];
    let d = &args[1];

    println!("N: {}", n);
    println!("d: {}", d);
}
