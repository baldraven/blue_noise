use honeycomb::core::cmap::CMap2;
use honeycomb::core::prelude::{CMapBuilder, Vertex2};
use honeycomb::render::App;
use std::collections::HashMap;

/// Extracts the vertices of the Voronoi cells from the pixel grid.
/// A vertex is only valid if there are 3 or more unique colors (0 for boundary).
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
        // FIXME: A little problem with saturating_sub : lines and column 0 are considered as outside the boundary
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
        .map(|&(nx, ny)| {
            // Check if neighbor is in bounds or at boundary
            if nx > 0 && ny > 0 && nx < res as u32 && ny < res as u32 {
                grid[ny as usize * res + nx as usize]
            } else {
                0 // Use 0 to represent None/boundary
            }
        })
        .collect();

        unique_colors.sort();
        unique_colors.dedup();

        if unique_colors.len() >= 3 {
            vertex_map.entry(unique_colors.clone()).or_insert((x, y));

            if current_color > 0 {
                // 0 is the boundary color
                let color_vertice = &mut color_vertices[current_color - 1];
                if !color_vertice.contains(&unique_colors) {
                    color_vertice.push(unique_colors);
                }
            }
        }
    }
}

pub fn calculate_face_centroid(
    vertices: &[Vec<usize>],
    vertex_map: &HashMap<Vec<usize>, (u32, u32)>,
) -> (f32, f32) {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for vertex in vertices {
        let pos = vertex_map.get(vertex).unwrap();
        sum_x += pos.0 as f32;
        sum_y += pos.1 as f32;
    }
    (sum_x / vertices.len() as f32, sum_y / vertices.len() as f32)
}

pub fn sort_vertices_by_angle(
    vertices: &mut [Vec<usize>],
    centroid: (f32, f32),
    vertex_map: &HashMap<Vec<usize>, (u32, u32)>,
) {
    vertices.sort_by(|a, b| {
        let pos_a = vertex_map.get(a).unwrap();
        let pos_b = vertex_map.get(b).unwrap();
        let angle_a = (pos_a.0 as f32 - centroid.0).atan2(pos_a.1 as f32 - centroid.1);
        let angle_b = (pos_b.0 as f32 - centroid.0).atan2(pos_b.1 as f32 - centroid.1);
        angle_b.partial_cmp(&angle_a).unwrap() // Swapped a and b to reverse order
    });
}

/// Generates a combinatorial map from a pixel grid by sewing darts between vertices.
/// Each face in the map corresponds to a color region in the pixel grid.
pub fn generate_mesh(pixels: &[usize], num_colors: usize) -> Result<(), &'static str> {
    let res = (pixels.len() as f64).sqrt() as usize;
    let mut color_vertices: Vec<Vec<Vec<usize>>> = vec![Vec::new(); num_colors];
    let mut vertex_map: HashMap<Vec<usize>, (u32, u32)> = HashMap::new();
    let mut vertices_id: HashMap<Vec<usize>, u32> = HashMap::new();
    let mut edges: HashMap<(&[usize], &[usize]), u32> = HashMap::new();

    extract_voronoi_cell_vertices(pixels, res, &mut color_vertices, &mut vertex_map);

    let mut map: CMap2<f32> = CMapBuilder::default().build().unwrap();

    let mut dart_id = 1;

    // Process each face (color region)
    for face_vertices in color_vertices.iter_mut() {
        // Sort vertices around face centroid for consistent orientation
        // TODO: use a topological algorithm instead
        let face_centroid = calculate_face_centroid(face_vertices, &vertex_map);
        sort_vertices_by_angle(face_vertices, face_centroid, &vertex_map);

        // Add darts for this face. at the end we need the exact number of darts or it will panic
        map.add_free_darts(face_vertices.len());

        // Process each vertex pair to insert and sew the darts
        for i in 0..face_vertices.len() {
            let current_vertex = &face_vertices[i];
            let next_vertex = &face_vertices[(i + 1) % face_vertices.len()];

            // Recording for the beta2 sewing later on

            // Add vertex geometry if not already added
            if !vertices_id.contains_key(current_vertex) {
                vertices_id.insert(current_vertex.to_vec(), dart_id);
                let (x, y) = vertex_map[current_vertex];
                let scaled_pos =
                    Vertex2::from((x as f32 * 5.0 / res as f32, y as f32 * 5.0 / res as f32));
                map.force_write_vertex(dart_id, scaled_pos);
            }

            edges.insert((current_vertex, next_vertex), dart_id);
            // Sew to opposite dart by checking if it exists in the edges map
            if let Some(&opposite_dart) = edges.get(&(next_vertex, current_vertex)) {
                map.force_two_sew(opposite_dart, dart_id);
            }
            // Sew to previous dart in face
            if i > 0 {
                map.force_one_sew(dart_id - 1, dart_id);
            }

            dart_id += 1;
        }

        // Close the face by sewing first and last darts
        map.force_one_sew(dart_id - 1, dart_id - face_vertices.len() as u32);
    }

    // Visualize the result
    let mut render_app = App::default();
    render_app.add_capture(&map);
    render_app.run();

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
            vec![
                vec![0, 1, 2],
                vec![0, 1, 5],
                vec![1, 3, 5],
                vec![1, 2, 4],
                vec![1, 3, 4],
            ],
            vec![vec![0, 1, 2], vec![0, 1, 2, 4], vec![0, 2, 4]],
            vec![vec![1, 3, 5], vec![0, 3, 5], vec![1, 3, 4], vec![0, 3, 4]],
            vec![vec![1, 2, 4], vec![1, 3, 4], vec![0, 2, 4], vec![0, 3, 4]],
            vec![vec![0, 1, 5], vec![1, 3, 5], vec![0, 3, 5]],
        ];

        let mut expected_vertex_map = HashMap::new();
        expected_vertex_map.insert(vec![1, 3, 5], (11, 4));
        expected_vertex_map.insert(vec![1, 2, 4], (2, 7));
        expected_vertex_map.insert(vec![0, 3, 5], (15, 4));
        expected_vertex_map.insert(vec![1, 3, 4], (9, 7));
        expected_vertex_map.insert(vec![0, 1, 2], (4, 0));
        expected_vertex_map.insert(vec![0, 1, 2, 4], (1, 7));
        expected_vertex_map.insert(vec![0, 3, 4], (15, 14));
        expected_vertex_map.insert(vec![0, 2, 4], (0, 8));
        expected_vertex_map.insert(vec![0, 1, 5], (10, 0));

        assert_eq!(color_vertices, expected_color_vertices);
        assert_eq!(vertex_map, expected_vertex_map);
    }
}
