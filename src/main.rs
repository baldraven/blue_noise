use blue_noise::*;
use std::fs::File;
use std::io::Write;
use std::{env, process};
pub mod plot;

fn main() {
    // Collecting input
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Processing
    let points = generate_points(&config).unwrap_or_else(|err| {
        println!("Problem generating points: {err}");
        process::exit(1);
    });

     let pixels = generate_cells(&points, &config).unwrap_or_else(|err| {
        println!("Problem running JFA: {err}");
        process::exit(1);
    });

    // Plotting
    //plot::plot_points(&points);
    plot::plot_heatmap_with_points(pixels, &points, (config.x, config.y));

    // Writing to file
  /*   let mut file = File::create("points.csv").expect("Unable to create file");
    for (x, y) in points {
        file.write_all(format!("{},{}\n", x, y).as_bytes())
            .expect("Unable to write data");
    }
    println!("Points written to points.csv"); */
}
