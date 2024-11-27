use honeycomb::core::prelude::CMapBuilder;
use honeycomb::render::App;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

fn stub_edges() {
    // Generate a vector of tuples as per the specified proportions
    let v = generate_random_vec(100);

    // Map to store which integers appear in which tuples
    let mut int_to_tuples: HashMap<i32, Vec<usize>> = HashMap::new();

    // Populate the map
    for (i, tuple) in v.iter().enumerate() {
        for &num in tuple {
            int_to_tuples.entry(num).or_default().push(i);
        }
    }

    // Set to track connected pairs to avoid duplicates
    let mut connected_pairs: HashSet<(usize, usize)> = HashSet::new();

    // Process each tuple
    for (i, tuple) in v.iter().enumerate() {
        // Collect potential candidates for connection
        let mut candidates = HashSet::new();
        for &num in tuple {
            if let Some(indices) = int_to_tuples.get(&num) {
                for &j in indices {
                    if i != j {
                        candidates.insert(j);
                    }
                }
            }
        }

        // Check each candidate for at least two common integers
        for &j in &candidates {
            if have_two_in_common(tuple, &v[j]) {
                let pair = if i < j { (i, j) } else { (j, i) };
                if connected_pairs.insert(pair) {
                    println!("Connecting tuples {} and {}", i, j);
                }
            }
        }
    }
}

// Function to check if two tuples have at least two integers in common
fn have_two_in_common(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a.intersection(&set_b).count() >= 2
}

// Function to generate a vector of tuples as per the specified proportions
fn generate_random_vec(size: usize) -> Vec<Vec<i32>> {
    let mut rng = thread_rng();
    let mut v = Vec::new();

    // 70% of the elements have 3 elements
    let size_3 = (size as f64 * 0.7).round() as usize;
    for _ in 0..size_3 {
        v.push(generate_tuple(3, &mut rng));
    }

    // 20% of the elements have 4 elements
    let size_4 = (size as f64 * 0.2).round() as usize;
    for _ in 0..size_4 {
        v.push(generate_tuple(4, &mut rng));
    }

    // Remaining 10% have sizes between 1 and 6
    let size_remaining = size - size_3 - size_4;
    for _ in 0..size_remaining {
        let random_size = rng.gen_range(1..=6);
        v.push(generate_tuple(random_size, &mut rng));
    }

    // Shuffle the vector for randomness
    v.shuffle(&mut rng);

    v
}

// Helper function to generate a tuple of `size` with random integers
fn generate_tuple(size: usize, rng: &mut ThreadRng) -> Vec<i32> {
    let mut tuple = Vec::new();
    for _ in 0..size {
        tuple.push(rng.gen_range(0..100));
    }
    tuple
}

pub fn generate_mesh(_pixels: &Vec<usize>) -> Result<(), &'static str> {
    stub_edges();

    let map = CMapBuilder::default().n_darts(5).build().unwrap();

    map.force_write_vertex(0, (0.0, 0.0));
    map.force_write_vertex(1, (0.0, 1.0));
    map.force_write_vertex(2, (1.0, 1.0));
    map.force_write_vertex(3, (1.0, 0.0));
    map.force_write_vertex(4, (2.0, 0.0));
    map.force_write_vertex(5, (2.0, 1.0));

    map.force_one_link(0, 1);
    map.force_one_link(1, 2);
    map.force_one_link(2, 3);
    map.force_one_link(3, 0);
    map.force_one_link(3, 4);
    map.force_one_link(4, 5);
    map.force_one_link(5, 2);

    let mut render_app = App::default();
    render_app.add_capture(&map);
    render_app.run();

    Ok(())
}
