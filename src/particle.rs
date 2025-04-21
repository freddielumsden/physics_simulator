use macroquad::prelude::*;
use ::glam::Vec2;

pub struct Particle {
    pub color: Color,
    pub radius: f32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    // pub boundary_cor: f32, // COR that the particle has with boundaries  https://en.wikipedia.org/wiki/Coefficient_of_restitution
    // Setting boundary_cor to anything below 1.0 will affect momentum output
}

impl Particle {
    pub fn update(&mut self, screen: [f32; 2]) {
        self.vel += self.acc;
        self.pos += self.vel;

        // collision w boundary detection
        if self.pos[0]+self.radius > screen[0] {
            self.vel[0] = -self.vel[0];
            self.pos[0] = screen[0]-self.radius;
        } else if self.pos[0]-self.radius < 0.0 {
            self.vel[0] = -self.vel[0];
            self.pos[0] = self.radius+0.1;
        }
        if self.pos[1]+self.radius > screen[1]{
            self.vel[1] = -self.vel[1];
            self.pos[1] = screen[1]-self.radius;
        } else if self.pos[1]-self.radius < 0.0 {
            self.vel[1] = -self.vel[1];
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


    pub fn collide(&mut self, ball_2: &mut Self) {
        // It's okay to check current frame because balls have just been updated but not displayed, so can still stop them from being inside each other for this frame
        let distance = ((self.pos[0]-ball_2.pos[0]).powf(2.0)+(self.pos[1]-ball_2.pos[1]).powf(2.0)).sqrt(); 
        if distance <= self.radius + ball_2.radius {
            let v_1 = self.vel;
            let v_2 = ball_2.vel;

            // Resolve vels
            self.vel += (v_2 - v_1).dot(ball_2.pos - self.pos) / distance / distance * (ball_2.pos - self.pos);
            ball_2.vel += (v_1 - v_2).dot(self.pos - ball_2.pos) / distance / distance * (self.pos - ball_2.pos);

            // Removes overlap
            // TODO: Make sure this overlap removal doesn't push one of the balls a bit off screen, can cause problems
            let overlap = (self.radius + ball_2.radius) - distance;
            let dir = (self.pos - ball_2.pos).clamp_length(overlap/2.0, overlap/2.0);
            self.pos += dir;
            ball_2.pos -= dir;
        }  
    }
}