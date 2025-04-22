use ::glam::Vec2;
use macroquad::prelude::*;

mod particle;

const G: f32 = 0.2; // Gravity constant
const BG_COLOR: Color = BLACK;
const GRID_SIZE: [i32; 2] = [10, 10];

// https://mq.agical.se/release-web.html how to build for html
// https://mq.agical.se/ch1-first-program.html#publish-on-the-web-if-you-want

#[macroquad::main("WAZZA")]
async fn main() {
    let color_choices = [RED, BLUE, YELLOW, ORANGE];
    let mut particles: Vec<particle::Particle> = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            particles.push(particle::Particle {
                color: color_choices[i % 4],
                radius: 20.0,
                pos: Vec2::new(
                    (i as f32) * ((screen_width() as f32) / 10.0),
                    (j as f32) * ((screen_height() as f32) / 10.0),
                ),
                vel: Vec2::new(i as f32, j as f32),
                acc: Vec2::new(0.0, G),
            });
        }
    }
    loop {
        clear_background(BG_COLOR); //Screen is cleared whether or not function is called so no performance reduction
        let screen = [screen_width(), screen_height()];

        for p in 0..particles.len() {
            // Physics update all balls
            particles[p].update(screen);
        }

        // Ball to ball collision detection
        for p in 0..particles.len() {
            for other_p in p + 1..particles.len() {
                let (left, right) = particles.split_at_mut(other_p);
                left[p].collide(&mut right[0]);
            }
        }

        let mut total_momentum = 0.0;
        for p in 0..particles.len() {
            draw_circle(
                particles[p].pos[0],
                particles[p].pos[1],
                particles[p].radius,
                particles[p].color,
            );

            total_momentum += particles[p].vel.length();
        }
        let frame_time = format!("Frame length: {:.2}ms", get_frame_time()*1000.0);
        let momentum_out = format!("Total momentum: {:.2} kg m/s", total_momentum);
        draw_fps();
        draw_text(&frame_time[..], 0.0, 35.0, 30.0, WHITE);
        draw_text(&&momentum_out[..], 0.0, 55.0, 30.0, WHITE);
        next_frame().await
    }
}
