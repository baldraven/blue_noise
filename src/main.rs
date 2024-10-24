use std::env;
use std::process;
use std::fs::File;
use std::io::Write;

use blue_noise::*;

use plotly::{Plot, Scatter, HeatMap,Layout};
use plotly::common::{Mode, ColorScalePalette};


fn _create_heatmap(data: Vec<usize>, x: usize, y: usize, nb_points: usize) {
    // Ensure the input data is exactly x * y in size
    assert_eq!(data.len(), x * y, "Data size must aequal x * y.");

    // Reshape the data into a 2D grid (Vec<Vec<usize>>)
    let mut grid: Vec<Vec<usize>> = vec![vec![0; y]; x];
    for i in 0..x {
        for j in 0..y {
            grid[i][j] = data[i * y + j];
        }
    }

    // Convert the grid to a 2D f64 array (required by HeatMap)
    let grid_f64: Vec<Vec<f64>> = grid.into_iter()
        .map(|row| row.into_iter().map(|v| v as f64).collect())
        .collect();

    let colorscale = ColorScalePalette::RdBu;
    let heatmap = HeatMap::new_z(grid_f64)
        .zmax(nb_points as f64)
        .zmin(-(nb_points as f64))
        .zmid(0.)
        .color_scale(colorscale.into());

    // Create the plot
    let mut plot = Plot::new();
    plot.add_trace(heatmap);

    let layout = Layout::new().height(1024).width(1024).auto_size(false);
    plot.set_layout(layout);
     
    // Display the plot in the browser
    plot.show();
}

pub fn plot_heatmap_with_points(data: Vec<usize>, points: &Vec<(f64, f64)>, x: usize, y: usize, nb_points: usize, config: &Config) {
    assert_eq!(data.len(), x * y, "Data size must equal x * y.");

    // Reshape the data into a 2D grid (Vec<Vec<usize>>)
    let mut grid: Vec<Vec<usize>> = vec![vec![0; y]; x];
    for i in 0..x {
        for j in 0..y {
            grid[i][j] = data[i * y + j];
        }
    }

    // Convert the grid to a 2D f64 array (required by HeatMap)
    let grid_f64: Vec<Vec<f64>> = grid.into_iter()
        .map(|row| row.into_iter().map(|v| v as f64).collect())
        .collect();

    // Create the heatmap
    let colorscale = ColorScalePalette::RdBu;
    let heatmap = HeatMap::new_z(grid_f64)
        .zmax(nb_points as f64)
        .zmin(-(nb_points as f64))
        .zmid(0.)
        .color_scale(colorscale.into());

        let normalized_points: Vec<(f64, f64)> = points.iter()
        .map(|(px, py)| (*px * x as f64 / config.x, *py * y as f64 / config.y))  // Scale points to match the heatmap resolution
        .collect();

    // Prepare the points for scatter plot
    let x_list: Vec<f64> = normalized_points.iter().map(|(x, _)| *x).collect();
    let y_list: Vec<f64> = normalized_points.iter().map(|(_, y)| *y).collect();
    let scatter = Scatter::new(x_list, y_list)
        .mode(Mode::Markers)
        .marker(plotly::common::Marker::new().size(10).color("#000000")); // Black markers for points

    // Create the plot and add both heatmap and scatter trace
    let mut plot = Plot::new();
    plot.add_trace(heatmap);
    plot.add_trace(scatter);

    // Define layout
    let layout = Layout::new()
        .height(1024)
        .width(1024)
        .auto_size(false);

    plot.set_layout(layout);

    // Show the plot in the browser
    plot.show();
}

pub fn plot_points(points: &Vec<(f64, f64)>) {
    let x_list = points.iter().map(|(x, _)| *x).collect();
    let y_list = points.iter().map(|(_, y)| *y).collect();   
    let mut plot = Plot::new();
    let trace = Scatter::new(x_list, y_list).mode(Mode::Markers);
    plot.add_trace(trace);
    let layout = Layout::new().height(1024).width(1024).auto_size(false);
    plot.set_layout(layout);
    plot.show();
    println!("Points plotted");
}

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

    plot_points(&points);
    plot_heatmap_with_points(pixels, &points, 512, 512, points.len(), &config);


    // Writing to file
    let mut file = File::create("points.csv").expect("Unable to create file");
    for (x, y) in points {
        file.write_all(format!("{},{}\n", x, y).as_bytes()).expect("Unable to write data");
    }
    println!("Points written to points.csv");

}


