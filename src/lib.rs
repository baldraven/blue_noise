pub struct Config {
    pub n: i64,
    pub d: i64
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

fn best_grid_dimensions(n: usize, x: usize, y: usize) -> (usize, usize) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec!["".to_string(), "100".to_string(), "1".to_string()];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.n, 100);
        assert_eq!(config.d, 1);
    }

    #[test]
    fn test_grid_dimension() {
        let (r, c) = best_grid_dimensions(16, 8, 8);
        assert_eq!(r, 4);
        assert_eq!(c, 4);
        
        let (r, c) = best_grid_dimensions(16, 1, 44);
        assert_eq!(r, 1);
        assert_eq!(c, 16);
    }

}
