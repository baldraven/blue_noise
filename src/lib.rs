pub mod cli;
mod jfa_cpu;
mod jfa_wgpu;
mod mode1;
mod mode2;
mod mode3;
mod plot;

use std::fs::File;
use std::io::Write;

pub fn generate_points(cli: &cli::Cli) -> Result<Vec<(f64, f64)>, &'static str> {
    match cli.mode {
        cli::Mode::GridWithN => Ok(mode1::generate_points(
            cli.n,
            cli.x as usize,
            cli.y as usize,
        )),
        cli::Mode::GridWithD => Ok(mode2::generate_points(cli.d, cli.x, cli.y)),
        cli::Mode::PoissonDisk => Ok(mode3::generate_points(cli.d, cli.x, cli.y)),
    }
}

pub fn generate_cells(
    points: &Vec<(f64, f64)>,
    cli: &cli::Cli,
) -> Result<Vec<usize>, &'static str> {
    match cli.jfa_mode {
        cli::JfaMode::None => Ok(vec![]),
        cli::JfaMode::Gpu => {
            println!("Generating cells using GPU with resolution {}...", cli.res);
            jfa_wgpu::main(points, (cli.x, cli.y))
        }
        cli::JfaMode::Cpu => {
            println!("Generating cells using CPU with resolution {}...", cli.res);
            jfa_cpu::jfa(points, (cli.x, cli.y))
        }
    }
}

pub fn handle_output(cli: &cli::Cli, points: &Vec<(f64, f64)>, pixels: Option<&Vec<usize>>) {
    // Export points to a CSV file if specified
    if let Some(ref export_path) = cli.export {
        let mut file = File::create(export_path).expect("Unable to create file");
        for (x, y) in points {
            file.write_all(format!("{},{}\n", x, y).as_bytes())
                .expect("Unable to write data");
        }
        println!("Points written to {}", export_path.display());
    }

    if matches!(cli.plot, cli::PlotMode::Points) {
        println!("Plotting points...");
        plot::plot_points(points);
    }

    if let Some(pixels) = pixels {
        if matches!(cli.plot, cli::PlotMode::Jfa) {
            println!("Plotting cells...");
            plot::plot_heatmap_with_points(pixels, points, (cli.x, cli.y));
        }
    }
}
