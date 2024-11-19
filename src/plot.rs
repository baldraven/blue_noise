use plotly::common::{ColorScalePalette, Mode};
use plotly::{HeatMap, Layout, Plot, Scatter};

pub fn plot_heatmap_with_points(
    data: &Vec<usize>,
    points: &Vec<(f64, f64)>,
    config_dimension: (f64, f64),
) {
    let reso = (data.len() as f64).sqrt() as usize;

    // Reshape the data into a 2D grid (Vec<Vec<usize>>)
    let mut grid: Vec<Vec<usize>> = vec![vec![0; reso]; reso];
    for i in 0..reso {
        for j in 0..reso {
            grid[i][j] = data[i * reso + j];
        }
    }

    // Convert the grid to a 2D f64 array (required by HeatMap)
    let grid_f64: Vec<Vec<f64>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|v| v as f64).collect())
        .collect();

    let colorscale = ColorScalePalette::Viridis;
    let heatmap = HeatMap::new_z(grid_f64).color_scale(colorscale.into());

    let normalized_points: Vec<(f64, f64)> = points
        .iter()
        .map(|(px, py)| {
            (
                *px * reso as f64 / config_dimension.0,
                *py * reso as f64 / config_dimension.1,
            )
        }) // Scale points to match the heatmap resolution
        .collect();

    // Prepare the points for scatter plot
    let x_list: Vec<f64> = normalized_points.iter().map(|(x, _)| *x).collect();
    let y_list: Vec<f64> = normalized_points.iter().map(|(_, y)| *y).collect();
    let scatter = Scatter::new(x_list, y_list)
        .mode(Mode::Markers)
        .marker(plotly::common::Marker::new().size(40).color("#000000")); // Black markers for points

    let mut plot = Plot::new();
    plot.add_trace(heatmap);
    plot.add_trace(scatter);

    let layout = Layout::new().height(2048).width(2048).auto_size(false);
    plot.set_layout(layout);

    plot.show();
}

pub fn plot_points(points: &Vec<(f64, f64)>) {
    let x_list = points.iter().map(|(x, _)| *x).collect();
    let y_list = points.iter().map(|(_, y)| *y).collect();
    let mut plot = Plot::new();

    let trace = Scatter::new(x_list, y_list)
        .mode(Mode::Markers)
        .marker(plotly::common::Marker::new().size(40)); // Black markers for points
    plot.add_trace(trace);
    let layout = Layout::new().height(2048).width(2048).auto_size(false);

    plot.set_layout(layout);
    plot.show();
}
