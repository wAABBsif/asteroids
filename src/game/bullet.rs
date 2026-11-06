use macroquad::{math::*, *};
use std::f32::consts::PI;

use crate::game::{asteroid::AsteroidManager, score::ScoreManager, wrap::Wrap};

const RADIUS: f32 = 0.05;
const LENGTH: f32 = 0.35;
const FADE_START_TIME: f32 = 1.6;
const COLOR: color::Color = color::BLUE;

const SPEED: f32 = 8.0;
const LIFETIME: f32 = 2.0;

const MAX_COUNT: usize = 32;

#[derive(Copy, Clone)]
pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub time_left: f32,
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

    pub fn update(&mut self, delta: f32, asteroid_manager: &mut AsteroidManager,
        score_manager: &mut ScoreManager)
    {
        if !self.is_active {
            return;
        }

        self.position += self.velocity * delta;
        self.handle_wrap();
        self.handle_collision(asteroid_manager, score_manager);

        self.time_left -= delta; 
        if self.time_left <= 0.0 {
            self.is_active = false;
        }
    }

    fn handle_collision(&mut self, asteroid_manager: &mut AsteroidManager,
        score_manager: &mut ScoreManager)
    {
        let colliding = asteroid_manager.get_asteroid_at_point(self.position);
        if colliding.is_some() {
            let colliding = colliding.unwrap();
            asteroid_manager.destroy_asteroid(colliding, score_manager);
            self.is_active = false;
        }
    }

    pub fn draw(&self) {
        if !self.is_active {
            return;
        }

        let alpha = clamp(self.time_left / (LIFETIME - FADE_START_TIME), 0.0, 1.0);
        let color = COLOR.with_alpha(alpha);

        let point = crate::gfx::world_to_screen_pos(self.position);
        let vertices = [
            crate::gfx::world_to_screen_pos(self.position + self.get_right_direction() * RADIUS),
            crate::gfx::world_to_screen_pos(self.position - self.get_right_direction() * RADIUS),
            crate::gfx::world_to_screen_pos(self.position - self.velocity.normalize() * LENGTH)
        ];

        shapes::draw_triangle(vertices[0], vertices[1], vertices[2], color);
        shapes::draw_circle(point.x, point.y, RADIUS * crate::gfx::pixels_per_unit(), color);
    }

    pub fn get_right_direction(&self) -> Vec2 {
        self.velocity.normalize().rotate(Vec2::from_angle(PI * 0.5))
    }
}

impl Wrap for Bullet {
    fn get_wrap_position(&self) -> Vec2 {
        return self.position;
    }

    fn set_wrap_position(&mut self, value: Vec2) {
        self.position = value;
    }

    fn get_size(&self) -> f32 {
        return RADIUS + LENGTH;
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

    pub fn create_bullet(&mut self, position: Vec2, angle: f32) -> Option<usize> {
        if self.bullets[self.head].is_active {
            return None
        }
        self.bullets[self.head] = Bullet::new(position, angle);
        let result = self.head;
        self.head = (self.head + 1) % MAX_COUNT;
        Some(result)
    }

    pub fn get_bullet(&mut self, id: usize) -> Option<&mut Bullet> {
        if !self.bullets[id].is_active {
            return None;
        }
        Some(&mut self.bullets[id])
    }

    pub fn update(&mut self, delta: f32, asteroid_manager: &mut AsteroidManager,
        score_manager: &mut ScoreManager)
    {
        for b in &mut self.bullets {
            b.update(delta, asteroid_manager, score_manager);
        }
    }

    pub fn draw(&self) {
        for b in self.bullets {
            b.draw();
        }
    }
}
