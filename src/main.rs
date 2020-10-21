// Approximating pi using Monte Carlo methods

// The Rng trait defines methods that random number generates implement
use rand::Rng;

fn main() {
    let total_iterations: u64 = 1_000_000;
    println!("circle to square ratio: pi = {}", circle_inside_square(total_iterations));
    println!("buffons needle: pi = {}", buffons_needle(total_iterations));
    println!("random walk: pi = {}", random_walk(1000, 10_000));
}

fn circle_inside_square(iterations: u64) -> f64 {
    // 1. Have a circle enclosed by a square with sides equal to the diameter of the circle
    // 2. Generate a random set of points on the square
    // 3. Area of the circle is pi*r^2 = pi/4 with r = 0.5
    // 4. Area of square = 1 x 1
    // 5. Divide area of circle by square we get pi / 4
    // 6. pi/4 ~ Ncircle/Ntotal
    // 7. pi ~ 4*Ncircle/Ntotal

    let radius = 1_f64;
    let mut inside_counter = 0_f64;
    
    // Get seed for random number generation
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        // Treat 0, 0 as the center so boundaries of positions x and y are +- the radius
        let pos_x = rng.gen_range(-radius, radius);
        let pos_y = rng.gen_range(-radius, radius);
        
        // If Euclidean distance is less than the radius than it's inside the circle
        if f64::sqrt(pos_x.powf(2_f64) + pos_y.powf(2_f64)) < radius {
            inside_counter += 1_f64;
        }
    }

    // Return approximation to pi 
    (4_f64 * inside_counter) / (iterations as f64)
}

fn buffons_needle(iterations: u64) -> f64 {
    // If a needle of length l is dropped n times on a surface on which parallel lines...
    // ...are drawn t units appart, and if x of those comes to rest crossing a line...
    // ...then pi ~ 2nl/xt
    
    let needle_length = 1_f64;
    let parallel_width = 1_f64;
    let two_pi = 6.28318530718_f64;

    let mut cross_counter = 0_f64;

    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        // Only care about the x position since the y position doesn't affect the outcome
        let needle_start_x = rng.gen_range(0_f64, parallel_width);

        let angle = rng.gen_range(0_f64, two_pi);
        let needle_end_x = needle_start_x + needle_length * f64::cos(angle);

        // If end of needle is outside of width then it has crossed a line
        if needle_end_x < 0_f64 || needle_end_x > parallel_width {
            cross_counter += 1_f64;
        }
    }

    (2_f64 * (iterations as f64) * needle_length) / (cross_counter * parallel_width) 
}

fn random_walk(steps: u64, walks: u64) -> f64 {
    // 1. Start a walk at position 0
    // 2. Generate a number between 0 and 1
    // 3. If number is less than 0.5, move position of x in the positive direction
    // 4. Else move it in the negative direction
    // 5. Calculate absolute distance from origin
    // 6. Do this step number of times and cumulatively add the absolute distance
    // 7. Do this walk number of times
    // 8. Average the number of absolute distances
    // 9. pi ~ 2 * steps / average_distance^2 

    let mut sum_of_abs_distances = 0_f64;

    let mut rng = rand::thread_rng();
    for _ in 0..walks {
        let mut position = 0_f64;
        for _ in 0..steps {
            let flip = rng.gen_range(0_f64, 1_f64);
    
            if flip < 0.5f64 {
                position += 1_f64;
            } else {
                position -= 1_f64;
            }
        }
        // Distance from origin
        let abs_distance = position.abs();
        sum_of_abs_distances += abs_distance;
    }

    let average_sum_of_abs_distances = sum_of_abs_distances / (walks as f64);

    // pi = 2 * n / (d_avg^2)
    (2_f64 * (steps as f64)) / (average_sum_of_abs_distances.powf(2_f64))
}