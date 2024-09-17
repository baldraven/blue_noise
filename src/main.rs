use std::env;
use std::process;
//use std::fs;

struct Config {
    n: i64,
    d: i64
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: n d, where n is the number of points and d is the minimal distance between points.");
        }

        let n = args[1].clone().parse();
        let d = args[2].clone().parse();

        match (n, d) {
            (Ok(n), Ok(d)) => Ok(Config {n, d}),
            _              => Err("n and d must be numbers")
        }
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    println!("N: {}", config.n);
    println!("d: {}", config.d);



}
