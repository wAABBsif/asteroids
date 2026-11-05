use std::f32::NAN;

use macroquad::{math::*, *};

use crate::game::wrap::Wrap;

const SPAWN_RADIUS: f32 = 1.0;
const DECREASE_RADIUS: f32 = 0.3;
const MIN_RADIUS: f32 = 0.4;

const MIN_SPEED_FACTOR: f32 = 0.1;
const MAX_SPEED_CONSTANT: f32 = 2.0;
const MAX_SPEED_COEFFICIENT: f32 = 1.0;

const COLOR: color::Color = color::GRAY;

const MAX_COUNT: usize = 48;

#[derive(Copy, Clone)]
pub struct Asteroid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub is_active: bool,
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

    pub fn is_point_inside(&self, point: Vec2) -> bool {
        return self.position.distance_squared(point) <= self.radius * self.radius
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

    fn create_asteroid(&mut self, position: Vec2, radius: f32) -> Option<&Asteroid>
    {
        if self.asteroids[self.head].is_active {
            return None
        }

        let angle = rand::RandomRange::gen_range(-std::f32::consts::PI, std::f32::consts::PI);
        let speed_factor = rand::RandomRange::gen_range(MIN_SPEED_FACTOR, 1.0);
        self.asteroids[self.head] = Asteroid::new(position, radius, angle, speed_factor);
        let result = &self.asteroids[self.head];
        self.head = (self.head + 1) % MAX_COUNT;
        Some(result)
    }

    pub fn spawn_asteroid(&mut self) -> Option<&Asteroid> {
        let perimeter = (crate::gfx::world_size().x + crate::gfx::world_size().y) * 2.0;
        let perimeter_point = rand::gen_range(0.0, perimeter);
        let position: Vec2;
        if perimeter_point < crate::gfx::world_size().x {
            let perimeter_point = perimeter_point - crate::gfx::world_size().x * 0.5;
            position = vec2(perimeter_point, f32::INFINITY);
        } else if perimeter_point < crate::gfx::world_size().x + crate::gfx::world_size().y {
            let perimeter_point = perimeter_point - crate::gfx::world_size().x * 1.5;
            position = vec2(f32::INFINITY, perimeter_point);
        } else if perimeter_point < perimeter - crate::gfx::world_size().y {
            let perimeter_point =
                perimeter_point - crate::gfx::world_size().x * 1.5 - crate::gfx::world_size().y;
            position = vec2(perimeter_point, f32::NEG_INFINITY);
        } else {
            let perimeter_point = perimeter_point
                - crate::gfx::world_size().x * 2.0 - crate::gfx::world_size().y * 1.5;
            position = vec2(f32::NEG_INFINITY, perimeter_point);
        }

        self.create_asteroid(position, SPAWN_RADIUS)
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

    pub fn get_asteroid_at_point(&self, point: Vec2) -> Option<&Asteroid> {
        for asteroid in &self.asteroids {
            if asteroid.is_point_inside(point) {
                return Some(asteroid)
            }
        }
        None
    }
}
