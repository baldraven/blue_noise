use honeycomb::core::cmap::CMap2;
use honeycomb::core::prelude::CMapBuilder;
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

            if current_color > 0 { // 0 is the boundary color
                let color_vertice = &mut color_vertices[current_color - 1];
                if !color_vertice.contains(&unique_colors) {
                    color_vertice.push(unique_colors);
                }
            }
        }
    }
}

pub fn calculate_face_centroid(vertices: &[Vec<usize>]) -> (f32, f32) {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for vertex in vertices {
        sum_x += vertex[0] as f32;
        sum_y += vertex[1] as f32;
    }
    (sum_x / vertices.len() as f32, sum_y / vertices.len() as f32)
}

pub fn sort_vertices_by_angle(vertices: &mut [Vec<usize>], centroid: (f32, f32)) {
    vertices.sort_by(|a, b| {
        let angle_a = (a[0] as f32 - centroid.0).atan2(a[1] as f32 - centroid.1);
        let angle_b = (b[0] as f32 - centroid.0).atan2(b[1] as f32 - centroid.1);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
}

pub fn insert_halfedge<'a>(
    map: &mut CMap2<f32>,
    halfedges: &mut HashMap<(&'a [usize], &'a [usize]), u32>,
    vertex_map: &HashMap<Vec<usize>, (u32, u32)>,
    he_count: &mut u32,
    face_vertices: &'a [Vec<usize>],
    i: usize,
) {
    let u = &face_vertices[i];
    let v = &face_vertices[(i + 1) % face_vertices.len()];

    *he_count += 1;
    halfedges.insert((u, v), *he_count);

    let x = vertex_map[u].0 as f32;
    let y = vertex_map[u].1 as f32;
    map.force_write_vertex(*he_count, (x, y));

    if let Some(&he_idx) = halfedges.get(&(v, u)) {
        map.force_two_sew(he_idx, *he_count);
    }
}

pub fn generate_mesh(pixels: &[usize], num_colors: usize) -> Result<(), &'static str> {
    let res = (pixels.len() as f64).sqrt() as usize;
    let mut color_vertices: Vec<Vec<Vec<usize>>> = vec![Vec::new(); num_colors];
    let mut vertex_map: HashMap<Vec<usize>, (u32, u32)> = HashMap::new();
    extract_voronoi_cell_vertices(pixels, res, &mut color_vertices, &mut vertex_map);

    let mut halfedges: HashMap<(&[usize], &[usize]), u32> = HashMap::new();
    let mut map: CMap2<f32> = CMapBuilder::default().n_darts(500).build().unwrap();

    let mut he_count = 0;

    for (_face_idx, face_vertices) in color_vertices.iter_mut().enumerate() {
        let face_centroid = calculate_face_centroid(face_vertices);
        sort_vertices_by_angle(face_vertices, face_centroid);

        insert_halfedge(&mut map, &mut halfedges, &vertex_map, &mut he_count, face_vertices, 0);
        for i in 1..face_vertices.len() {
            insert_halfedge(&mut map, &mut halfedges, &vertex_map, &mut he_count, face_vertices, i);
            map.force_one_link(he_count-1, he_count);
        }
        map.force_one_link(he_count, he_count- face_vertices.len() as u32 + 1); // not sure about the +1
    }

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

    #[test]
    fn test_sort_vertices_by_angle() {
        let mut vertices = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let centroid = ((1.0 + 2.0 + 3.0) / 3.0, (2.0 + 3.0 + 1.0) / 3.0);
        sort_vertices_by_angle(&mut vertices, centroid);
        assert!(
            (vertices == vec![vec![3, 1], vec![1, 2], vec![2, 3]])
                || (vertices == vec![vec![2, 3], vec![3, 1], vec![1, 2]])
                || (vertices == vec![vec![1, 2], vec![2, 3], vec![3, 1]])
        );
    }

    #[test]
    fn test_calculate_face_centroid() {
        let vertices = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let centroid = calculate_face_centroid(&vertices);
        assert_eq!(centroid, ((1.0 + 2.0 + 3.0) / 3.0, (2.0 + 3.0 + 1.0) / 3.0));
    }
}
