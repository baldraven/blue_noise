/* use honeycomb::core::prelude::CMapBuilder;
use honeycomb::render::App; */
use std::collections::HashMap;

/// Extracts the vertices of the Voronoi cells from the pixel grid.
/// A vertex is only valid if there are 3 or more unique colors.
/// The keys of `Vertex_map` are the ordered Vec<usize> of the adjacent colors of the vertices, and are used in `color_vertices`, that tracks the vertices of each Voronoi cell.
pub fn extract_voronoi_cell_vertices(
    grid: &[usize],
    res: usize,
    color_vertices: &mut [Vec<Vec<usize>>],
    vertex_map: &mut HashMap<Vec<usize>, (u32, u32)>,
) {
    for (idx, &current_color) in grid.iter().enumerate() {
        let x = (idx % res) as u32;
        let y = (idx / res) as u32;

        // Collect unique colors in the 8-neighborhood
        let mut unique_colors: Vec<usize> = [
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

        unique_colors.sort();
        unique_colors.dedup();

        if unique_colors.len() >= 3 {
            vertex_map.entry(unique_colors.clone()).or_insert((x, y));

            let color_vertice = &mut color_vertices[current_color - 1];
            if !color_vertice.contains(&unique_colors) {
                color_vertice.push(unique_colors);
            }
        }
    }
}

pub fn generate_mesh(pixels: &[usize], num_colors: usize) -> Result<(), &'static str> {
    let res = (pixels.len() as f64).sqrt() as usize;
    let mut color_vertices: Vec<Vec<Vec<usize>>> = vec![Vec::new(); num_colors];
    let mut vertex_map: HashMap<Vec<usize>, (u32, u32)> = HashMap::new();
    extract_voronoi_cell_vertices(pixels, res, &mut color_vertices, &mut vertex_map);
    dbg!(color_vertices);
    dbg!(vertex_map);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_voronoi_cell_vertices() {
        let pixels = vec![
            2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 5, 5,
            5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1,
            1, 1, 5, 5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 2, 2, 2, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 3, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 2, 2, 1, 1,
            1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3,
            3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3,
        ];
        let num_colors = 5;
        let res = (pixels.len() as f64).sqrt() as usize;
        let mut color_vertices: Vec<Vec<Vec<usize>>> = vec![Vec::new(); num_colors];
        let mut vertex_map: HashMap<Vec<usize>, (u32, u32)> = HashMap::new();
        extract_voronoi_cell_vertices(&pixels, res, &mut color_vertices, &mut vertex_map);

        let expected_color_vertices = vec![
            vec![vec![1, 3, 5], vec![1, 2, 4], vec![1, 3, 4]],
            vec![vec![1, 2, 4]],
            vec![vec![1, 3, 5], vec![1, 3, 4]],
            vec![vec![1, 2, 4], vec![1, 3, 4]],
            vec![vec![1, 3, 5]],
        ];

        let mut expected_vertex_map = HashMap::new();
        expected_vertex_map.insert(vec![1, 3, 4], (9, 7));
        expected_vertex_map.insert(vec![1, 3, 5], (11, 4));
        expected_vertex_map.insert(vec![1, 2, 4], (1, 7));

        assert_eq!(color_vertices, expected_color_vertices);
        assert_eq!(vertex_map, expected_vertex_map);
    }
}
