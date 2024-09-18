use std::env;
use std::process;

use blue_noise::Config;


fn main() {
    
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    println!("N: {}", config.n);
    println!("d: {}", config.d);
        

}


