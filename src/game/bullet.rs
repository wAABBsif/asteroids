use macroquad::{math::*, *};
use std::f32::consts::PI;

use crate::collections::SwapArray;
use crate::game::{asteroid::AsteroidManager, score::ScoreManager, screen::{self, EdgeBehavior}};

const RADIUS: f32 = 0.05;
const LENGTH: f32 = 0.35;
const FADE_START_TIME: f32 = 1.6;
const COLOR: color::Color = color::BLUE;

const SPEED: f32 = 8.0;
const LIFETIME: f32 = 2.0;

const MAX_COUNT: usize = 32;

pub struct BulletManager {
    bullets: SwapArray<Bullet, MAX_COUNT>,
}

impl BulletManager {
    pub fn new() -> BulletManager {
        BulletManager {
            bullets: SwapArray::default()
        }
    }

    pub fn create_bullet(&mut self, position: Vec2, angle: f32) {
        if self.bullets.is_full() {
            return;
        }
        self.bullets.add(Bullet::new(position, angle));
    }

    pub fn update(&mut self, delta: f32, asteroid_manager: &mut AsteroidManager,
        score_manager: &mut ScoreManager)
    {
        for b in &mut self.bullets {
            b.update(delta, asteroid_manager, score_manager);
        }

        self.bullets.retain(|&b| !b.queue_removal);
    }

    pub fn draw(&self) {
        for b in &self.bullets {
            b.draw();
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Bullet {
    queue_removal: bool,
    pub position: Vec2,
    pub velocity: Vec2,
    pub time_left: f32,
}

impl Bullet {
    pub fn new(position: Vec2, angle: f32) -> Bullet {
        Bullet {
            queue_removal: false,
            position: position,
            velocity: Vec2::from_angle(angle) * SPEED,
            time_left: LIFETIME,
        }
    }

    pub fn update(&mut self, delta: f32, asteroid_manager: &mut AsteroidManager,
        score_manager: &mut ScoreManager)
    {
        self.position += self.velocity * delta;
        screen::handle_edge_behavior(self, self.position, RADIUS);
        self.handle_collision(asteroid_manager, score_manager);

        self.time_left -= delta; 
        if self.time_left <= 0.0 {
            self.queue_removal = true;
        }
    }

    fn handle_collision(&mut self, asteroid_manager: &mut AsteroidManager,
        score_manager: &mut ScoreManager)
    {
        let colliding = asteroid_manager.get_asteroid_at_point(self.position);
        if colliding.is_some() {
            let colliding = colliding.unwrap();
            asteroid_manager.destroy_asteroid(colliding, score_manager);
            self.queue_removal = true;
        }
    }

    pub fn draw(&self) {
        let alpha = clamp(self.time_left / (LIFETIME - FADE_START_TIME), 0.0, 1.0);
        let color = COLOR.with_alpha(alpha);

        let point = screen::world_to_screen_offset(self.position);
        let vertices = [
            screen::world_to_screen_offset(self.position + self.right_direction() * RADIUS),
            screen::world_to_screen_offset(self.position - self.right_direction() * RADIUS),
            screen::world_to_screen_offset(self.position - self.velocity.normalize() * LENGTH)
        ];

        shapes::draw_triangle(vertices[0], vertices[1], vertices[2], color);
        shapes::draw_circle(point.x, point.y, RADIUS * screen::pixels_per_unit(), color);
    }

    pub fn right_direction(&self) -> Vec2 {
        self.velocity.normalize().rotate(Vec2::from_angle(PI * 0.5))
    }
}

impl EdgeBehavior for Bullet {
    fn on_hit_screen_edge(&mut self, axis: screen::Axis) {
        screen::screen_wrap(axis, &mut self.position, RADIUS);
    }
}
