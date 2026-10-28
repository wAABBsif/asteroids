use macroquad::{math::*, *};

const RADIUS: f32 = 0.05;
const COLOR: color::Color = color::BLUE;

const SPEED: f32 = 8.0;
const LIFETIME: f32 = 2.0;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    time_left: f32,
    is_active: bool
}

impl Bullet {
    pub fn new(position: Vec2, angle: f32) -> Bullet {
        Bullet {
            position: position,
            velocity: Vec2::from_angle(angle) * SPEED,
            time_left: LIFETIME,
            is_active: true,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.is_active {
            return;
        }

        self.position += self.velocity * delta;
        self.time_left -= delta;
        if self.time_left <= 0.0 {
            self.is_active = false;
        }
    }

    pub fn draw(&self) {
        if !self.is_active {
            return;
        }

        let point = crate::gfx::world_to_screen_pos(self.position);
        shapes::draw_circle(point.x, point.y, RADIUS * crate::gfx::pixels_per_unit(), COLOR);
    }
}
