use macroquad::prelude::*;
use ::glam::Vec2;

const G: f32 = 0.09; // Gravity constant
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
            self.pos[0] = self.radius;
        }
        if self.pos[1]+self.radius > screen[1]{
            self.vel[1] = -self.vel[1]*self.boundary_cor;
            self.pos[1] = screen[1]-self.radius;
        } else if self.pos[1]-self.radius < 0.0 {
            self.vel[1] = -self.vel[1]*self.boundary_cor;
            self.pos[1] = self.radius;
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
    let distance =((ball_1.pos[0]-ball_2.pos[0]).powf(2.0)+(ball_1.pos[1]-ball_2.pos[1]).powf(2.0)).sqrt(); 
    if distance < ball_1.radius + ball_2.radius {
        /*
        let ball_1_vels = [ball_1.vel[0], ball_1.vel[1]];
        [ball_1.vel[0], ball_1.vel[1]] = [ball_2.vel[0], ball_2.vel[1]];
        [ball_2.vel[0], ball_2.vel[1]] = ball_1_vels;       
        
        // Chat GPT generated the 7 lines of code after this one. But there was an error which after going through it mathermatically on paper I spotted. Remember this moment!!!
        
        
        
        let x_proportion = (ball_1.pos[0]-ball_2.pos[0])/distance;
        let y_proportion = (ball_1.pos[1]-ball_2.pos[1])/distance;
        
        ball_1.pos[0] += x_proportion * overlap;
        ball_1.pos[1] += y_proportion * overlap;
        ball_2.pos[0] -= x_proportion * overlap;
        ball_2.pos[1] -= y_proportion * overlap;
        
        
        
        
        */
        
        // TODO: Optimise this maths
        // Wikipedia didn't work, it was this that worked in the end https://www.vobarian.com/collisions/2dcollisions2.pdf Very good document from 2009!
        let normal_vector = ball_2.pos - ball_1.pos;
        // To convert a vector to its normal just divide each component by the magnitude
        // Bear in mind that powi is not deterministic -> might need to switch
        let unit_normal = normal_vector / (normal_vector[0].powi(2) + normal_vector[1].powi(2)).powf(0.5);
        let unit_tangent = Vec2::new(-unit_normal[1], unit_normal[0]);
        
        let v_1_in_norm_dir = unit_normal.dot(ball_1.vel);
        let v_2_in_norm_dir = unit_normal.dot(ball_2.vel);
        let v_1_in_tan_dir = unit_tangent.dot(ball_1.vel);
        let v_2_in_tan_dir = unit_tangent.dot(ball_2.vel);

        let new_v_1_in_norm_dir = v_2_in_norm_dir;
        let new_v_2_in_norm_dir = v_1_in_norm_dir;

        ball_1.vel = new_v_1_in_norm_dir * unit_normal + v_1_in_tan_dir * unit_tangent;
        ball_2.vel = new_v_2_in_norm_dir * unit_normal + v_2_in_tan_dir * unit_tangent;
        
        // TODO: Optimise this maths -> trig function use
        // Moves the balls out of each other. This is my maths btw!
        let overlap = ((ball_1.radius + ball_2.radius) - distance)/2.0;

        let m = (ball_1.pos[1]-ball_2.pos[1])/(ball_1.pos[0]-ball_2.pos[0]);
        let theta = m.atan().abs();
        let y_change = overlap*theta.sin();
        let x_change = overlap*theta.cos();
        
        // Making sure that you do the adding and subtracting the right way round, don't just make the overlap larger
        if ball_1.pos[0] <= ball_2.pos[0] {
            ball_1.pos[0] -= x_change;
            ball_2.pos[0] += x_change;
            ball_1.pos[1] -= y_change;
            ball_2.pos[1] += y_change;
        } else {
            ball_1.pos[0] += x_change;
            ball_2.pos[0] -= x_change;
            ball_1.pos[1] += y_change;
            ball_2.pos[1] -= y_change;
        }
    }   



}

// TODO: ADD BALL ON BALL COLLISION
// USE GRID SYSTEM TO MAKE MORE EFFICIENT
#[macroquad::main("WAZZA")]
async fn main() {
    let color_choices = [RED, BLUE, YELLOW, ORANGE];
    let mut balls: Vec<Ball> = Vec::new();
    for i in 0..2 {
        for j in 0..5 {
            balls.push(Ball {
            color: color_choices[i%4],
            radius: 20.0,
            pos: Vec2::new((i as f32)*((screen_width() as f32)/10.0), (j as f32)*((screen_height() as f32)/10.0)),
            vel: Vec2::new((i as f32)*10.0, (j as f32)*10.0),
            acc: Vec2::new(0.0, G),
            boundary_cor: 0.9,
        });
    }
    }
    loop {
        // You have to do passes in order to have less overlap
        clear_background(BG_COLOR); //Screen is cleared whether or not function is called so no performance reduction
        let screen = [screen_width(), screen_height()];
        
        for ball in 0..balls.len() { // Physics update all balls
            balls[ball].update(screen);
        }
        for _ in 0..5 {
        // Ball to ball collision detection
            for y_grid_pos in 0..GRID_SIZE[1] { //10x10 grid
                let y_bounds = [screen[1]*(y_grid_pos as f32/10.0), screen[1]*((y_grid_pos as f32 + 1.0)/10.0)];
                for x_grid_pos in 0..GRID_SIZE[0] {
                    let x_bounds = [screen[0]*(x_grid_pos as f32/10.0), screen[0]*((x_grid_pos as f32 + 1.0)/10.0)];
                    let mut grid_balls: Vec<usize> = Vec::new();
                    for ball in 0..balls.len() {
                        if (balls[ball].pos[0]+balls[ball].radius >= x_bounds[0] && balls[ball].pos[0]-balls[ball].radius <= x_bounds[1]) && (balls[ball].pos[1]+balls[ball].radius >= y_bounds[0] && balls[ball].pos[1]+balls[ball].radius <= y_bounds[1]) {
                            grid_balls.push(ball);
                        }

                    }
                    for ball in 0..grid_balls.len() {
                        for other_ball in ball+1..grid_balls.len() {
                            let (left, right) = balls.split_at_mut(grid_balls[other_ball]);
                            collide(&mut left[grid_balls[ball]], &mut right[0]);
                        }
                    }
                }
            };
        for ball in 0..balls.len() { // Physics update all balls
            draw_circle(balls[ball].pos[0], balls[ball].pos[1], balls[ball].radius, balls[ball].color);
        }
        
    }
    draw_fps();
            
    next_frame().await
    }
}