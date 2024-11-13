mod jfa_cpu;
mod jfa_wgpu;
mod mode1;
mod mode2;
mod mode3;

pub struct Config {
    pub n_or_d: f64,
    pub mode: u32,
    pub x: f64,
    pub y: f64,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("Usage: mode n_or_d x y");
        }

        let mode = args[1].clone().parse();
        let x = args[3].clone().parse();
        let y = args[4].clone().parse();
        let n_or_d = args[2].clone().parse();

        match (mode, n_or_d, x, y) {
            (Ok(mode), Ok(n_or_d), Ok(x), Ok(y)) => Ok(Config { mode, n_or_d, x, y }),
            _ => Err("Error parsing arguments"),
        }
    }
}

pub fn generate_points(config: &Config) -> Result<Vec<(f64, f64)>, &'static str> {
    match config.mode {
        1 => Ok(mode1::generate_points(
            config.n_or_d as usize,
            config.x as usize,
            config.y as usize,
        )),
        2 => Ok(mode2::generate_points(
            config.n_or_d as f64,
            config.x,
            config.y,
        )),
        3 => Ok(mode3::generate_points(
            config.n_or_d as f64,
            config.x,
            config.y,
        )),
        _ => Err("Invalid mode"),
    }
}

pub fn generate_cells_gpu(
    points: &Vec<(f64, f64)>,
    config: &Config,
) -> Result<Vec<usize>, &'static str> {
    jfa_wgpu::main(points, (config.x, config.y))
}

pub fn generate_cells_cpu(
    points: &Vec<(f64, f64)>,
    config: &Config,
) -> Result<Vec<usize>, &'static str> {
    jfa_cpu::jfa(points, (config.x, config.y))
