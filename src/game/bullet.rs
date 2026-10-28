use macroquad::{math::*, *};

const RADIUS: f32 = 0.05;
const COLOR: color::Color = color::BLUE;

const SPEED: f32 = 8.0;
const LIFETIME: f32 = 1.0;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    time: f32
}

impl Bullet {
    pub fn new(position: Vec2, angle: f32) -> Bullet {
        Bullet {
            position: position,
            velocity: Vec2::from_angle(angle) * SPEED,
            time: LIFETIME
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        self.time -= delta;
    }

    pub fn draw(&self) {
        let point = crate::gfx::world_to_screen_pos(self.position);
        shapes::draw_circle(point.x, point.y, RADIUS * crate::gfx::pixels_per_unit(), COLOR);
    }

    pub fn is_life_over(&self) -> bool {
        self.time <= 0.0
    }
}
