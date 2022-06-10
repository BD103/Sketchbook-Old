/*
This example is broken. I will come back to this for better entity collision at some point.
 */

use manyvecs::Vec2f;

const RAY_START: Vec2f = Vec2f { x: 0.0, y: 0.0 };

const RAY_END: Vec2f = Vec2f { x: 2.0, y: 1.0 };

fn main() {
    let ray_slope = RAY_END - RAY_START;
    let mut ray_pos = RAY_START.clone();

    // Find step size
    let mut step_size = Vec2f::new(1.0, 1.0);

    if ray_slope.x < 0.0 {
        step_size.x = -1.0;
    }

    if ray_slope.y < 0.0 {
        step_size.y = -1.0;
    }

    let step_size = step_size;

    // Calculate if x == 1
    let xa = ray_slope.y / ray_slope.x;

    // Calculate if y == 1
    let ya = ray_slope.x / ray_slope.y;

    let x_dist = (1.0 + xa * xa).sqrt();
    let y_dist = (1.0 + ya * ya).sqrt();

    println!("X: {}, Y: {}", x_dist, y_dist);

    if x_dist < y_dist {
        ray_pos.x += step_size.x;
        ray_pos.y += xa;
    } else {
        ray_pos.x += ya;
        ray_pos.y += step_size.y;
    }

    println!("{}", ray_pos);
}
