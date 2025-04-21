use macroquad::prelude::*;
use ::glam::Vec2;

const G: f32 = 0.0; // Gravity constant
const BG_COLOR: Color = BLACK;
const GRID_SIZE: [i32; 2] = [10, 10];

struct Ball {
    color: Color,
    radius: f32,
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    boundary_cor: f32, // COR that the ball has with boundaries  https://en.wikipedia.org/wiki/Coefficient_of_restitution
}

impl Ball {
    fn update(&mut self, screen: [f32; 2]) {
        self.vel += self.acc;
        self.pos += self.vel;

        // collision w boundary detection
        if self.pos[0]+self.radius > screen[0] {
            self.vel[0] = -self.vel[0]*self.boundary_cor;
            self.pos[0] = screen[0]-self.radius;
        } else if self.pos[0]-self.radius < 0.0 {
            self.vel[0] = -self.vel[0]*self.boundary_cor;
            self.pos[0] = self.radius+0.1;
        }
        if self.pos[1]+self.radius > screen[1]{
            self.vel[1] = -self.vel[1]*self.boundary_cor;
            self.pos[1] = screen[1]-self.radius;
        } else if self.pos[1]-self.radius < 0.0 {
            self.vel[1] = -self.vel[1]*self.boundary_cor;
            self.pos[1] = self.radius+0.1;
        }
        //collision w cursor detection
        let cursor = mouse_position();
        if cursor.0 > self.pos[0]-self.radius && cursor.0 < self.pos[0]+self.radius {
            if cursor.1 > self.pos[1]-self.radius && cursor.1 < self.pos[1]+self.radius {
                self.vel[1] = -self.vel[1];
                self.vel[0] = -self.vel[0];
            }
        }
    }
}

fn collide(ball_1: &mut Ball, ball_2: &mut Ball) {
    // It's okay to check current frame because balls have just been updated but not displayed, so can still stop them from being inside each other for this frame
    let distance = ((ball_1.pos[0]-ball_2.pos[0]).powf(2.0)+(ball_1.pos[1]-ball_2.pos[1]).powf(2.0)).sqrt(); 
    if distance <= ball_1.radius + ball_2.radius {
        let v_1 = ball_1.vel;
        let v_2 = ball_2.vel;

        // Resolve vels
        ball_1.vel += (v_2 - v_1).dot(ball_2.pos - ball_1.pos) / distance / distance * (ball_2.pos - ball_1.pos);
        ball_2.vel += (v_1 - v_2).dot(ball_1.pos - ball_2.pos) / distance / distance * (ball_1.pos - ball_2.pos);
    }  
}


#[macroquad::main("WAZZA")]
async fn main() {
    let color_choices = [RED, BLUE, YELLOW, ORANGE];
    let mut balls: Vec<Ball> = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            balls.push(Ball {
            color: color_choices[i%4],
            radius: 20.0,
            pos: Vec2::new((i as f32)*((screen_width() as f32)/10.0), (j as f32)*((screen_height() as f32)/10.0)),
            vel: Vec2::new(i as f32, j as f32),
            acc: Vec2::new(0.0, G),
            boundary_cor: 0.9,
        });
    }
    }
    loop {
        clear_background(BG_COLOR); //Screen is cleared whether or not function is called so no performance reduction
        let screen = [screen_width(), screen_height()];
        
        for ball in 0..balls.len() { // Physics update all balls
            balls[ball].update(screen);
        }

        // Ball to ball collision detection
        for ball in 0..balls.len() {
            for other_ball in ball+1..balls.len() {
                let (left, right) = balls.split_at_mut(other_ball);
                collide(&mut left[ball], &mut right[0]);
            }
        }

        let mut total_momentum = 0.0;
        for ball in 0..balls.len() {
            draw_circle(balls[ball].pos[0], balls[ball].pos[1], balls[ball].radius, balls[ball].color);

            total_momentum += balls[ball].vel.length();
        }
        println!("{}", total_momentum);

        draw_fps();
        next_frame().await
    }
}