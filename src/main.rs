// Approximating pi using Monte Carlo methods

// The Rng trait defines methods that random number generates implement
use rand::Rng;

// Piston engine for points inside circle approximaiton
use piston_window::*;
// Need image to save previous frame and loaded onto next frame
use ::image;
// For loading font to display digits of pi
use ::find_folder;

fn main() {
    let steps: u64 = 100;
    let walks: u64 = 10_000;
    println!("random walk: pi = {}", random_walk(steps, walks));
    
    let total_iterations: u64 = 1_000_000;
    println!("buffons needle: pi = {}", buffons_needle(total_iterations));

    // This one has visuals using piston_window library
    // pi approximation is printed on the console
    circle_inside_square();
}

fn circle_inside_square() {
    // Display circle inside square pi approximation
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 540;
    const TEXT_HEIGHT: u32 = 28;

    let mut window: PistonWindow = WindowSettings::new("Approximating Pi", [WIDTH, HEIGHT])
            .exit_on_esc(true).build().unwrap();

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    // Create an image buffer to save previous frame and draw it again on next frame
    let mut canvas = image::ImageBuffer::new(WIDTH, HEIGHT);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
        &mut texture_context,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    // Set up font for text to show pi
    let assets = find_folder::Search::ParentsThenKids(3, 3)
    .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    // Monte carlo method for random points inside a circle:
    // 1. Have a circle enclosed by a square with sides equal to the diameter of the circle
    // 2. Generate a random set of points on the square
    // 3. Area of the circle is pi * r^2 with r = 0.5
    // 4. Area of square = 1 x 1
    // 5. Divide area of circle by square we get pi / 4
    // 6. pi / 4 ~ Ncircle / Ntotal
    // 7. pi ~ 4 * Ncircle / Ntotal

    // Counter for number of points in circle and total counter
    let mut inside_counter = 0_f64;
    let mut total_counter = 0_f64;
    
    println!("Displaying visuals for random points inside circle...");
    let mut rng = rand::thread_rng();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            // Clear display to white
            clear([1.0; 4], g);
            
            let rect = [0.0, 0.0, WIDTH as f64, (HEIGHT-TEXT_HEIGHT) as f64];
            ellipse(GREEN, rect, c.transform, g);
            
            
            let pos_x = rng.gen_range(0, WIDTH);
            let pos_y = rng.gen_range(0, HEIGHT-TEXT_HEIGHT);
            
            // Put generated square pixels into canvas
            for i in 0..5 {
                if pos_x + i < WIDTH {
                    for j in 0..5 {
                        if pos_y + j < HEIGHT-TEXT_HEIGHT {
                            canvas.put_pixel(pos_x + i, pos_y + j, image::Rgba([255, 0, 0, 255]));
                        }
                    }
                }
            }

            // Map pos_x and pos_y between -1 and 1 (circle of radius 1 with center [0, 0])
            let point_x = map(pos_x as f64, 0.0, WIDTH as f64, -1.0, 1.0);
            let point_y = map(pos_y as f64, 0.0, (HEIGHT-TEXT_HEIGHT) as f64, -1.0, 1.0);
            
            // If Euclidean distance is less than the radius than it's inside the circle
            let square_of_radius = 1; // square of 1 is 1
            if point_x.powf(2_f64) + point_y.powf(2_f64) < square_of_radius as f64 {
                inside_counter += 1_f64;
            }
            total_counter += 1_f64;
            
            // Update texture
            texture.update(&mut texture_context, &canvas).unwrap();
            image(&texture, c.transform, g);
            texture_context.encoder.flush(device);

            // Draw text for pi approximation
            let transform = c.transform.trans(10.0, 535.0);

            text::Text::new_color([0.0, 0.0, 1.0, 1.0], 28).draw(
                &format!("{}", (4_f64 * inside_counter) / (total_counter as f64)).to_string(),
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn map(val: f64, min: f64, max: f64, new_min: f64, new_max: f64) -> f64 {
    // Map val to new val based on new range
    let range = max - min;
    let new_range = new_max - new_min;
    (val / range) * new_range + new_min
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
    // 5. Do this step number of times
    // 6. Calculate absolute distance from origin and sum it cumulatively
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