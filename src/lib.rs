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

}
