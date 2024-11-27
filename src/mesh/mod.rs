use honeycomb::core::prelude::CMapBuilder;
use honeycomb::render::App;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

pub fn extract_voronoi_cell_vertices(grid: &Vec<usize>, res: usize, num_colors: usize) -> Vec<Vec<(u32, u32)>> {
    let mut color_vertices: Vec<Vec<(u32, u32)>> = vec![Vec::new(); num_colors];

    for (idx, &current_color) in grid.iter().enumerate() {
        let x = (idx % res) as u32;
        let y = (idx / res) as u32;

        // Collect unique colors in the 8-neighborhood
        let unique_colors: HashSet<usize> = [
            (x.saturating_sub(1), y.saturating_sub(1)),
            (x, y.saturating_sub(1)),
            (x + 1, y.saturating_sub(1)),
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x.saturating_sub(1), y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter_map(|&(nx, ny)| {
            // Check if neighbor is in bounds
            if nx < res as u32 && ny < res as u32 {
                Some(grid[ny as usize * res + nx as usize])
            } else {
                None
            }
        })
        .collect();

        // A vertex is only valid if there are 3 or more unique colors
        if unique_colors.len() >= 3 {
            let vertices = &mut color_vertices[(current_color - 1) as usize];
        
            // Check if this vertex is already close to an existing one using Manhattan distance
            let is_near_existing = vertices.iter().any(|&(vx, vy)| {
                let manhattan_dist = (vx as i32 - x as i32).abs() + (vy as i32 - y as i32).abs();
                manhattan_dist <= 3
            });
        
            // If not near an existing vertex, add it
            if !is_near_existing {
                vertices.push((x, y));
            }
        }
    }

    color_vertices
}

pub fn generate_mesh(pixels: &Vec<usize>, num_colors: usize) -> Result<(), &'static str> {
    let res = (pixels.len() as f64).sqrt() as usize;
    
    let vertices = extract_voronoi_cell_vertices(pixels, res, num_colors);
    dbg!(vertices);

    Ok(())
}
