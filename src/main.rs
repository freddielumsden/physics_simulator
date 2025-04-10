use macroquad::prelude::*;

const G: f32 = 0.09; // Gravity constant
const BG_COLOR: Color = BLACK;
const GRID_SIZE: [i32; 2] = [10, 10];

struct Ball {
    color: Color,
    radius: f32,
    x_pos: f32,
    y_pos: f32,
    x_vel: f32,
    y_vel: f32,
    x_acc: f32,
    y_acc: f32,
    boundary_cor: f32, // COR that the ball has with boundaries  https://en.wikipedia.org/wiki/Coefficient_of_restitution
}

impl Ball {
    fn update(&mut self, screen: [f32; 2]) {
        self.x_vel += self.x_acc;
        self.y_vel += self.y_acc;

        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;

        // collision w boundary detection
        if self.x_pos+self.radius > screen[0] {
            self.x_vel = -self.x_vel*self.boundary_cor;
            self.x_pos = screen[0]-self.radius;
        } else if self.x_pos-self.radius < 0.0 {
            self.x_vel = -self.x_vel*self.boundary_cor;
            self.x_pos = self.radius;
        }
        if self.y_pos+self.radius > screen[1]{
            self.y_vel = -self.y_vel*self.boundary_cor;
            self.y_pos = screen[1]-self.radius;
        } else if self.y_pos-self.radius < 0.0 {
            self.y_vel = -self.y_vel*self.boundary_cor;
            self.y_pos = self.radius;
        }
        //collision w cursor detection
        let cursor = mouse_position();
        if cursor.0 > self.x_pos-self.radius && cursor.0 < self.x_pos+self.radius {
            if cursor.1 > self.y_pos-self.radius && cursor.1 < self.y_pos+self.radius {
                self.y_vel = -self.y_vel;
                self.x_vel = -self.x_vel;
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
        

        let m = (ball_1.y_pos-ball_2.y_pos)/(ball_1.x_pos-ball_2.x_pos);
        let theta = m.atan().abs();
        let y_change = overlap*theta.sin();
        let x_change = overlap*theta.cos();

        // Making sure that you do the adding and subtracting the right way round, don't just make the overlap larger
        if ball_1.x_pos <= ball_2.x_pos {
            ball_1.x_pos -= x_change;
            ball_2.x_pos += x_change;
            ball_1.y_pos -= y_change;
            ball_2.y_pos += y_change;
        } else {
            ball_1.x_pos += x_change;
            ball_2.x_pos -= x_change;
            ball_1.y_pos += y_change;
            ball_2.y_pos -= y_change;
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
            x_pos: (i as f32)*((screen_width() as f32)/100.0),
            y_pos: (j as f32)*((screen_height() as f32)/100.0),
            x_vel: 30.0,
            y_vel: (i as f32),
            x_acc: 0.0,
            y_acc: G,
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
                        if (balls[ball].x_pos+balls[ball].radius >= x_bounds[0] && balls[ball].x_pos-balls[ball].radius <= x_bounds[1]) && (balls[ball].y_pos+balls[ball].radius >= y_bounds[0] && balls[ball].y_pos+balls[ball].radius <= y_bounds[1]) {
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
        
    }
    draw_fps();
            
    next_frame().await
    }
}