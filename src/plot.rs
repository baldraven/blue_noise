use plotly::{Plot, Scatter, HeatMap,Layout};
use plotly::common::{Mode, ColorScalePalette};

pub fn plot_heatmap_with_points(data: Vec<usize>, points: &Vec<(f64, f64)>, reso: usize, nb_points: usize, config_dimension: (f64, f64)) {
    assert_eq!(data.len(), reso * reso, "Data size must equal x * y.");

    // Reshape the data into a 2D grid (Vec<Vec<usize>>)
    let mut grid: Vec<Vec<usize>> = vec![vec![0; reso]; reso];
    for i in 0..reso {
        for j in 0..reso {
            grid[i][j] = data[i * reso + j];
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
        .map(|(px, py)| (*px * reso as f64 / config_dimension.0, *py * reso as f64 / config_dimension.1))  // Scale points to match the heatmap resolution
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
