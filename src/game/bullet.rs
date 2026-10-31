use macroquad::{math::*, *};
use std::f32::consts::PI;

const RADIUS: f32 = 0.05;
const LENGTH: f32 = 0.35;
const COLOR: color::Color = color::BLUE;

const SPEED: f32 = 8.0;
const LIFETIME: f32 = 2.0;

const MAX_COUNT: usize = 32;

#[derive(Copy, Clone)]
pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    time_left: f32,
    is_active: bool,
}

pub struct BulletManager {
    bullets: [Bullet; MAX_COUNT],
    head: usize,
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
        let vertices = [
            crate::gfx::world_to_screen_pos(self.position + self.get_right_direction() * RADIUS),
            crate::gfx::world_to_screen_pos(self.position - self.get_right_direction() * RADIUS),
            crate::gfx::world_to_screen_pos(self.position - self.velocity.normalize() * LENGTH)
        ];

        shapes::draw_triangle(vertices[0], vertices[1], vertices[2], COLOR);
        shapes::draw_circle(point.x, point.y, RADIUS * crate::gfx::pixels_per_unit(), COLOR);
    }

    pub fn get_right_direction(&self) -> Vec2 {
        self.velocity.normalize().rotate(Vec2::from_angle(PI * 0.5))
    }
}

impl BulletManager {
    pub fn new() -> BulletManager {
        BulletManager {
            bullets: [
                Bullet {
                    position: Vec2::NAN,
                    velocity: Vec2::NAN,
                    time_left: 0.0,
                    is_active: false,
                }; MAX_COUNT
            ],
            head: 0
        }
    }

    pub fn create_bullet(&mut self, position: Vec2, angle: f32) -> Option<&Bullet> {
        if self.bullets[self.head].is_active {
            return None
        }
        self.bullets[self.head] = Bullet::new(position, angle);
        let result = &self.bullets[self.head];
        self.head = (self.head + 1) % MAX_COUNT;
        Some(result)
    }

    pub fn update(&mut self, delta: f32) {
        for b in &mut self.bullets {
            b.update(delta);
        }
    }

    pub fn draw(&self) {
        for b in self.bullets {
            b.draw();
        }
    }
}
