use macroquad::prelude::*;

const G: f32 = 0.09; // Gravity constant
const DELTA: f32 = 1.0;
const BG_COLOR: Color = BLACK;

struct Ball {
    color: Color,
    radius: f32,
    x_pos: f32,
    y_pos: f32,
    x_init_pos: f32, // At the start of the most recent collision
    y_init_pos: f32,
    x_vel: f32,
    y_vel: f32,
    x_init_vel: f32, // At the start of the most recent collision
    y_init_vel: f32,
    x_acc: f32,
    y_acc: f32,
    time: f32, // (relative, resets to 0 after every collision)
    boundary_cor: f32, // COR that the ball has with the ground https://en.wikipedia.org/wiki/Coefficient_of_restitution
}

impl Ball {
    fn update(&mut self, screen: [f32; 2]) {
        self.time += DELTA;
        // We know the acceleration, we know the time and we know the initial velocity
        self.x_vel = self.x_init_vel + self.x_acc * self.time;
        self.y_vel = self.y_init_vel + self.y_acc * self.time;

        // We know u, v, t, a, find s (want to use most efficient equation)
        let displacement_x = (self.x_init_vel + self.x_vel)/2.0 * self.time;
        let displacement_y = (self.y_init_vel + self.y_vel)/2.0 * self.time;
        self.x_pos = self.x_init_pos + displacement_x;
        self.y_pos = self.y_init_pos + displacement_y;
        

        // collision w boundary detection
        if self.x_pos+self.radius > screen[0] {
            self.x_vel = -self.x_vel*self.boundary_cor;
            self.x_pos = screen[0]-self.radius;
            self.x_init_pos = self.x_pos;
            self.y_init_pos = self.y_pos;
            self.x_init_vel = self.x_vel;
            self.y_init_vel = self.y_vel;
            self.time = 0.0;
        } else if self.x_pos-self.radius < 0.0 {
            self.x_vel = -self.x_vel*self.boundary_cor;
            self.x_pos = self.radius;
            self.x_init_pos = self.x_pos;
            self.y_init_pos = self.y_pos;
            self.x_init_vel = self.x_vel;
            self.y_init_vel = self.y_vel;
            self.time = 0.0;
        }
        if self.y_pos+self.radius > screen[1]{
            self.y_vel = -self.y_vel*self.boundary_cor;
            self.y_pos = screen[1]-self.radius;
            self.x_init_pos = self.x_pos;
            self.y_init_pos = self.y_pos;
            self.x_init_vel = self.x_vel;
            self.y_init_vel = self.y_vel;
            self.time = 0.0;
        } else if self.y_pos-self.radius < 0.0 {
            self.y_vel = -self.y_vel*self.boundary_cor;
            self.y_pos = self.radius;
            self.x_init_pos = self.x_pos;
            self.y_init_pos = self.y_pos;
            self.x_init_vel = self.x_vel;
            self.y_init_vel = self.y_vel;
            self.time = 0.0;
        }
        //collision w cursor detection
        let cursor = mouse_position();
        if cursor.0 > self.x_pos-self.radius && cursor.0 < self.x_pos+self.radius {
            if cursor.1 > self.y_pos-self.radius && cursor.1 < self.y_pos+self.radius {
                self.y_vel = -self.y_vel;
                self.x_vel = -self.x_vel;
                self.x_init_pos = self.x_pos;
                self.y_init_pos = self.y_pos;
                self.x_init_vel = self.x_vel;
                self.y_init_vel = self.y_vel;
                self.time = 0.0;
            }
        }

    }
}

fn collide(ball_1: &mut Ball, ball_2: &mut Ball) {
    let distance =((ball_1.x_pos-ball_2.x_pos).powf(2.0)+(ball_1.y_pos-ball_2.y_pos).powf(2.0)).sqrt(); 
    if distance < ball_1.radius + ball_2.radius {
        let ball_1_vels = [ball_1.x_vel, ball_1.y_vel];
        [ball_1.x_vel, ball_1.y_vel] = [ball_2.x_vel, ball_2.y_vel];
        [ball_2.x_vel, ball_2.y_vel] = ball_1_vels;       
        
        // Chat GPT generated the 7 lines of code after this one. But there was an error which after going through it mathermatically on paper I spotted. Remember this moment!!!
        let overlap = ((ball_1.radius + ball_2.radius) - distance)/2.0;
        let x_proportion = (ball_1.x_pos-ball_2.x_pos)/distance;
        let y_proportion = (ball_1.y_pos-ball_2.y_pos)/distance;
        
        ball_1.x_pos += x_proportion * overlap;
        ball_1.y_pos += y_proportion * overlap;
        ball_2.x_pos -= x_proportion * overlap;
        ball_2.y_pos -= y_proportion * overlap;
        

        // let m = (ball_1.y_pos-ball_2.y_pos)/(ball_1.x_pos-ball_2.x_pos);


        ball_1.time = 0.0;
        ball_2.time = 0.0;
        ball_1.x_init_pos = ball_1.x_pos;
        ball_2.x_init_pos = ball_2.x_pos;
        ball_1.y_init_pos = ball_1.y_pos;
        ball_2.y_init_pos = ball_2.y_pos;
        ball_1.x_init_vel = ball_1.x_vel;
        ball_2.x_init_vel = ball_2.x_vel;
        ball_1.y_init_vel = ball_1.y_vel;
        ball_2.y_init_vel = ball_2.y_vel;
        
    }
}

// TODO: ADD BALL ON BALL COLLISION
// USE GRID SYSTEM TO MAKE MORE EFFICIENT
#[macroquad::main("WAZZA")]
async fn main() {
    let color_choices = [RED, BLUE, YELLOW];
    let mut balls: Vec<Ball> = Vec::new();
    for i in 0..100 {
        for j in 0..100 {
            balls.push(Ball {
            color: color_choices[i%3],
            radius: 4.0,
            x_pos: (i as f32)*((screen_width() as f32)/100.0),
            y_pos: (j as f32)*((screen_height() as f32)/100.0),
            x_init_pos: (i as f32)*((screen_width() as f32)/100.0),
            y_init_pos: (j as f32)*((screen_height() as f32)/100.0),
            x_vel: 30.0,
            y_vel: (i as f32),
            x_init_vel: 30.0,
            y_init_vel: (i as f32),
            x_acc: 0.0,
            y_acc: G,
            time: 0.0,
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
        
        for y_grid_pos in 0..10 { //10x10 grid
            let y_bounds = [screen[1]*(y_grid_pos as f32/10.0), screen[1]*((y_grid_pos as f32 + 1.0)/10.0)];
            for x_grid_pos in 0..10 {
                let x_bounds = [screen[0]*(x_grid_pos as f32/10.0), screen[0]*((x_grid_pos as f32 + 1.0)/10.0)];
                let mut grid_balls: Vec<usize> = Vec::new();
                for ball in 0..balls.len() {
                    if (balls[ball].x_pos+balls[ball].radius >= x_bounds[0] && balls[ball].x_pos-balls[ball].radius <= x_bounds[1]) && (balls[ball].y_pos >= y_bounds[0] && balls[ball].y_pos <= y_bounds[1]) {
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
            draw_circle(balls[ball].x_pos, balls[ball].y_pos, balls[ball].radius, balls[ball].color);
        }
        draw_fps();
            
        next_frame().await
    }
}