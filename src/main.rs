use std::env;
use std::process;
use std::fs::File;
use std::io::Write;

use blue_noise::*;

use plotly::{Plot, Scatter};
use plotly::common::Mode;

fn main() {
    
    // Collecting input
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });




    // Processing
    let (cols, rows) = best_grid_dimensions(config.n_or_d, config.x, config.y);
    let points = fit_points_in_rectangle(rows, cols, config.x, config.y); 



    // Plotting
    let x_list = points.iter().map(|(x, _)| *x).collect();
    let y_list = points.iter().map(|(_, y)| *y).collect();   
    let mut plot = Plot::new();
    let trace = Scatter::new(x_list, y_list).mode(Mode::Markers);
    plot.add_trace(trace);
    plot.show();
    println!("Points plotted");

    // Writing to file
    let mut file = File::create("points.csv").expect("Unable to create file");
    for (x, y) in points {
        file.write_all(format!("{},{}\n", x, y).as_bytes()).expect("Unable to write data");
    }
    println!("Points written to points.csv");

}


