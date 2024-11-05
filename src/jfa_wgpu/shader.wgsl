@group(0) @binding(0) var<storage, read_write> pixel_grid: array<u32>;
@group(0) @binding(1) var<uniform> step: u32;

const RESO: u32 = 512;

fn metric(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    let dx = (x1 - x2) * (x1 - x2);
    let dy = (y1 - y2) * (y1 - y2);
    return dx + dy;
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= RESO || y >= RESO) {
        return;
    }

    let index = x + y * RESO;
    let current_color = pixel_grid[index];
    
    // If this pixel is already colored, skip it
    if (current_color != 0) {
        return;
    }

    var best_color = 0u;
    var best_distance = 0xffffffffu; // Max possible distance
    
    for (var dx = -1; dx <= 1; dx++) {
        for (var dy = -1; dy <= 1; dy++) {
            let nx = x + u32(dx) * step;
            let ny = y + u32(dy) * step;
            if (nx < RESO && ny < RESO) {
                let neighbor_index = nx + ny * RESO;
                let neighbor_color = pixel_grid[neighbor_index];
                
                if (neighbor_color != 0) {
                    let distance = metric(x, y, nx, ny);
                    if (distance < best_distance) {
                        best_distance = distance;
                        best_color = neighbor_color;
                    }
                }
            }
        }
    }

    if (best_color != 0) {
        pixel_grid[index] = best_color;
    }
}