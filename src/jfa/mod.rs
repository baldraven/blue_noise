pub fn jfa(points: &Vec<(f64, f64)>, config: (f64, f64)) -> Result<Vec<usize>, &'static str> {
    const RESO: usize = 512;

    // let's normalize points position relative with the config box size x and y.
    let normal_points: Vec<(usize, usize)> = points.into_iter()
        .map(|(a, b)| ((a * RESO as f64 / config.0) as usize, (b * RESO as f64 / config.1) as usize))
        .collect();

    let mut pixel_grid = vec![0; (RESO * RESO) as usize];

    // Mark the initial points on the grid with their respective color (index + 1)
    for (i, point) in normal_points.iter().enumerate() {
        let color = i+1; // 0 means uncolored
        pixel_grid[point.0 + point.1*RESO] = color;
    }

    let mut k = normal_points.len() / 2 ;
    while k >= 1 {

        for x in 0..RESO {
            for y in 0..RESO {
                // Check the 8-neighborhood (jump in all directions) and update to the closest point
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let new_x = x as isize + dx * k as isize;
                        let new_y = y as isize + dy * k as isize;

                        // Ensure the new coordinates are within bounds
                        if new_x >= 0 && new_x < RESO as isize && new_y >= 0 && new_y < RESO as isize { //TODO: reduce imbrication
                            let new_idx = (new_x as usize) + (new_y as usize) * RESO;
                            let found_color = pixel_grid[new_idx];

                            if found_color == 0 {
                                continue;
                            }

                            let current_color = pixel_grid[x + y * RESO];
                            if current_color == 0 {
                                pixel_grid[x + y * RESO] = found_color;
                            }

                            // we're now in the case where we have two colors.
                            // we need to find the original coordinates of the two colors, in the normal_points vector, and calculate distances
                            
                            // complete here
                            // Get the original coordinates of the two colors
                            let point1 = normal_points[current_color - 1]; // current color
                            let point2 = normal_points[found_color - 1];   // found color

                            // Calculate the distance from the current pixel to each point
                            let dist1 = ((x as isize - point1.0 as isize).pow(2)
                                + (y as isize - point1.1 as isize).pow(2)) as f64;
                            let dist2 = ((x as isize - point2.0 as isize).pow(2)
                                + (y as isize - point2.1 as isize).pow(2)) as f64;

                            // Assign the color of the point that is closer
                            if dist2 < dist1 {
                                pixel_grid[x + y * RESO] = found_color;
                            }



                        }
                    }
                }
            }
        }


        k /= 2
    }

    Ok(pixel_grid)
}


mod tests{
    #[test]
    fn test_insert_pixel() {
        let points = vec![(1.0, 1.0)];
        let config = (2.0, 2.0);

        let pixel_grid = jfa(&points, config).unwrap();

        assert_eq!(pixel_grid[12], 0);
        assert_eq!(pixel_grid[512*256+256], 1);
    }

}