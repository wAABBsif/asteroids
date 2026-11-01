use std::f32::NAN;

use macroquad::{math::*, *};

use crate::game::wrap::Wrap;

const MAX_SPEED_CONSTANT: f32 = 2.0;
const MAX_SPEED_COEFFICIENT: f32 = 1.0;

const COLOR: color::Color = color::GRAY;

const MAX_COUNT: usize = 48;

#[derive(Copy, Clone)]
pub struct Asteroid {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    is_active: bool,
}

pub struct AsteroidManager {
    asteroids: [Asteroid; MAX_COUNT],
    head: usize,
}

impl Asteroid {
    pub fn new(position: Vec2, radius: f32, angle: f32, speed_factor: f32) -> Asteroid {
        Asteroid {
            position: position,
            velocity: Vec2::from_angle(angle) * speed_factor * Asteroid::max_speed(radius),
            radius: radius,
            is_active: true,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.is_active {
            return;
        }

        self.position += self.velocity * delta;
        self.handle_wrap();
    }

    pub fn draw(&self) {
        if !self.is_active {
            return;
        }

        let point = crate::gfx::world_to_screen_pos(self.position);
        shapes::draw_circle(point.x, point.y, self.radius * crate::gfx::pixels_per_unit(), COLOR);
    }

    fn max_speed(radius: f32) -> f32 {
        MAX_SPEED_CONSTANT + MAX_SPEED_COEFFICIENT / radius
    }
}

impl Wrap for Asteroid {
    fn get_wrap_position(&self) -> Vec2 {
        return self.position;
    }

    fn set_wrap_position(&mut self, value: Vec2) {
        self.position = value;
    }

    fn get_size(&self) -> f32 {
        return self.radius;
    }
}

impl AsteroidManager {
    pub fn new() -> AsteroidManager {
        AsteroidManager {
            asteroids: [
                Asteroid {
                    position: Vec2::NAN,
                    velocity: Vec2::NAN,
                    radius: NAN,
                    is_active: false,
                }; MAX_COUNT
            ],
            head: 0,
        }
    }

    pub fn create_asteroid(&mut self, position: Vec2, radius: f32, angle: f32, speed_factor: f32)
        -> Option<&Asteroid>
    {
        if self.asteroids[self.head].is_active {
            return None
        }

        self.asteroids[self.head] = Asteroid::new(position, radius, angle, speed_factor);
        let result = &self.asteroids[self.head];
        self.head = (self.head + 1) % MAX_COUNT;
        Some(result)
    }

    pub fn update(&mut self, delta: f32) {
        for a in &mut self.asteroids {
            a.update(delta);
        }
    }

    pub fn draw(&self) {
        for a in self.asteroids {
            a.draw();
        }
    }
}
