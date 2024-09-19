pub struct Config {
    pub n_or_d: usize,
    pub mode: u32,
    pub x: usize,
    pub y: usize,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("Usage: mode n_or_d x y");
        }

        let mode = args[1].clone().parse();
        let n_or_d = args[2].clone().parse();
        let x = args[3].clone().parse();
        let y = args[4].clone().parse();

        match (mode, n_or_d, x, y) {
            (Ok(mode), Ok(n_or_d), Ok(x), Ok(y)) => Ok(Config { mode, n_or_d, x, y }),
            _                                    => Err("Error parsing arguments")
        }
    }
}

pub fn best_grid_dimensions(n: usize, x: usize, y: usize) -> (usize, usize) {
    let target_aspect_ratio = x as f64 / y as f64;
    let mut best_r = 1;
    let mut best_c = n;
    let mut best_ratio_diff = f64::MAX;

    for r in 1..=n {
        if n % r == 0 {
            let c = n / r;

            // Calculate the aspect ratio for the current grid configuration
            let current_aspect_ratio = c as f64 / r as f64;

            // Calculate the difference from the target aspect ratio
            let ratio_diff = (current_aspect_ratio - target_aspect_ratio).abs();

            // Check if this configuration is closer to the target aspect ratio
            if ratio_diff < best_ratio_diff {
                best_ratio_diff = ratio_diff;
                best_r = r;
                best_c = c;
            }
        }
    }

    (best_c, best_r)
}

pub fn fit_points_in_rectangle(rows: usize, cols: usize, width: usize, height: usize) -> Vec<(f64, f64)> {
    let mut points = Vec::new();

    // Calculate the spacing between points
    let x_spacing = width as f64 / (cols + 1) as f64;
    let y_spacing = height as f64 / (rows + 1) as f64;

    for r in 0..rows {
        for c in 0..cols {
            // Calculate the coordinates of each point
            let x = (c + 1) as f64 * x_spacing;
            let y = (r + 1) as f64 * y_spacing;
            points.push((x, y));
        }
    }

    points
}

pub fn generate_grid_points(d: f64, width: f64,height: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::new();

    // Calculate how many points fit along the width and height based on distance `d`
    let num_points_x = (width / d).floor() as usize;
    let num_points_y = (height / d).floor() as usize;

    // Generate the points in a grid pattern
    for i in 0..=num_points_x {
        for j in 0..=num_points_y {
            let x = i as f64 * d;
            let y = j as f64 * d;
            points.push((x, y));
        }
    }

    points
}

pub fn generate_points(config: Config) -> Result<Vec<(f64, f64)>, &'static str> {
    match config.mode {
        1 => {
            let (cols, rows) = best_grid_dimensions(config.n_or_d, config.x, config.y);
            Ok(fit_points_in_rectangle(rows, cols, config.x, config.y))
        },
        2 => {
            Ok(generate_grid_points(config.n_or_d as f64, config.x as f64, config.y as f64))
        },
        _ => {
            Err("Invalid mode")
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_dimension() {
        let (r, c) = best_grid_dimensions(16, 8, 8);
        assert_eq!(r, 4);
        assert_eq!(c, 4);
        
        let (r, c) = best_grid_dimensions(16, 1, 44);
        assert_eq!(r, 1);
        assert_eq!(c, 16);
    }


    #[test]
    fn test_fit_points_in_rectangle() {
        let rows = 3;
        let cols = 4;
        let width = 10;
        let height = 5;

        let expected_points = vec![
            (2., 1.25), (4., 1.25), (6., 1.25), (8., 1.25),
            (2., 2.5),  (4., 2.5),  (6., 2.5),  (8., 2.5),
            (2., 3.75), (4., 3.75), (6., 3.75), (8., 3.75),
        ];

        let points = fit_points_in_rectangle(rows, cols, width, height);

        assert_eq!(points.len(), expected_points.len());

        for (point, expected) in points.iter().zip(expected_points.iter()) {
            assert!((point.0 - expected.0).abs() < f64::EPSILON);
            assert!((point.1 - expected.1).abs() < f64::EPSILON);
        }

    }


    #[test]
    fn test_generate_grid_points_count() {
        // Test with a 100x100 rectangle and minimum distance of 10
        let points = generate_grid_points(10.0, 100.0, 100.0);

        // Expecting a 11x11 grid (since 100 / 10 = 10, but we include 0)
        let expected_num_points = (10 + 1) * (10 + 1); // 11 * 11 = 121
        assert_eq!(points.len(), expected_num_points);
    }


    #[test]
    fn test_generate_grid_points_spacing() {
        // Test with a 30x30 rectangle and minimum distance of 10
        let points = generate_grid_points(10.0, 30.0, 30.0);

        // Ensure that points are spaced by at least `d = 10.0` along the grid
        for (i, &(x1, y1)) in points.iter().enumerate() {
            for (j, &(x2, y2)) in points.iter().enumerate() {
                if i != j {
                    let distance_x = (x2 - x1).abs();
                    let distance_y = (y2 - y1).abs();

                    // Points should be at least 10 units apart on the grid
                    assert!(distance_x >= 10.0 || distance_y >= 10.0,
                        "Points ({}, {}) and ({}, {}) are too close",
                        x1, y1, x2, y2);
                }
            }
        }
    }

    #[test]
    fn test_no_points_generated() {
        // Test where the distance is larger than the rectangle's dimensions
        let points = generate_grid_points(100.0, 50.0, 50.0);

        // Should return only the (0,0) point
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], (0.0, 0.0));
    }
}
