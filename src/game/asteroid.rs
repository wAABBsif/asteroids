use std::f32::{NAN, consts::PI};

use macroquad::{math::*, *};

use crate::game::{player::Player, score::ScoreManager, wrap::Wrap};

const SPAWN_RADIUS: f32 = 1.0;
const DECREASE_RADIUS: f32 = 0.3;
const MIN_RADIUS: f32 = 0.4;

const MIN_SPEED_FACTOR: f32 = 0.1;
const MAX_SPEED_CONSTANT: f32 = 2.0;
const MAX_SPEED_COEFFICIENT: f32 = 1.0;

const SCORE_RADIUS_FACTOR: f32 = 100.0;
const SCORE_COMBO_FACTOR: u64 = 10;

const ASTEROIDS_WAVE_MIN: f32 = 2.0;
const ASTEROIDS_WAVE_RATE: f32 = 1.0;
const ASTEROIDS_WAVE_MAX: u64 = 11;

const POINT_RADIUS_RATIO: f32 = 16.0;
const POINT_RADIUS_MARGIN: f32 = 0.125;
const BRIGHT_COLOR: color::Color = color::Color {
    r: 0.4,
    g: 0.4,
    b: 0.5,
    a: 1.0
};
const DARK_COLOR: color::Color = color::Color {
    r: 0.2,
    g: 0.2,
    b: 0.3,
    a: 1.0
};

const MAX_POINT_COUNT: usize = (POINT_RADIUS_RATIO * SPAWN_RADIUS) as usize;
const MAX_COUNT: usize = 48;

#[derive(Clone, Copy)]
pub struct Asteroid {
    is_active: bool,
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    shape: Shape,
}

#[derive(Clone, Copy)]
struct Shape {
    size: usize,
    points: [Vec2; MAX_POINT_COUNT],
}

pub struct AsteroidManager {
    asteroids: [Asteroid; MAX_COUNT],
    head: usize,
    wave_count: u64,
}

impl Asteroid {
    pub fn new(position: Vec2, radius: f32, angle: f32, speed_factor: f32) -> Asteroid {
        Asteroid {
            is_active: true,
            position: position,
            velocity: Vec2::from_angle(angle) * speed_factor * Asteroid::max_speed(radius),
            radius: radius,
            shape: Shape::new(radius)
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.is_active {
            return;
        }

        self.position += self.velocity * delta;
        self.handle_wrap();
    }

    pub fn draw(&self, player: &Player) {
        if !self.is_active {
            return;
        }

        for i in 0..self.shape.size {
            let pos = crate::gfx::world_to_screen_offset(self.position);
            let curr_point = self.shape.points[i];
            let next_point = self.shape.points[(i + 1) % self.shape.size];
            let triangle_pts = [
                pos,
                pos + crate::gfx::world_to_screen(curr_point),
                pos + crate::gfx::world_to_screen(next_point),
            ];
            let player_dir = (player.position - self.position).normalize();
            let normal = (curr_point + next_point) * 0.5;
            let lightness = (normal.dot(player_dir) + 1.0) * 0.5;
            let color = macroquad::color::Color {
                r: DARK_COLOR.r + (BRIGHT_COLOR.r - DARK_COLOR.r) * lightness,
                g: DARK_COLOR.g + (BRIGHT_COLOR.g - DARK_COLOR.g) * lightness,
                b: DARK_COLOR.b + (BRIGHT_COLOR.b - DARK_COLOR.b) * lightness,
                a: 1.0
            };
            shapes::draw_triangle(triangle_pts[0], triangle_pts[1], triangle_pts[2], color);

        }
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

impl Shape {
    fn new(radius: f32) -> Shape {
        let mut result = Shape {
            size: (radius * POINT_RADIUS_RATIO) as usize,
            points: [Vec2::NAN; MAX_POINT_COUNT]
        };
        for i in 0..result.size {
            let angle = (i as f32) / (result.size as f32) * 2.0 * PI;
            let dist = rand::RandomRange::gen_range(
                radius - POINT_RADIUS_MARGIN, radius + POINT_RADIUS_MARGIN);
            result.points[i] = Vec2::from_angle(angle) * dist;
        }

        result
    }
}

impl AsteroidManager {
    pub fn new() -> AsteroidManager {
        AsteroidManager {
            asteroids: [Asteroid {
                is_active: false,
                position: Vec2::NAN,
                velocity: Vec2::NAN,
                radius: NAN,
                shape: Shape {
                    size: 0,
                    points: [Vec2::NAN; MAX_POINT_COUNT]
                },
            }; MAX_COUNT],
            head: 0,
            wave_count: 0
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.get_asteroid_count() == 0 {
            self.initialize_wave();
        }
        for a in &mut self.asteroids {
            a.update(delta);
        }
    }

    pub fn draw(&self, player: &Player) {
        for a in &self.asteroids {
            a.draw(player);
        }
    }

    pub fn initialize_wave(&mut self) {
        let count = std::cmp::min((ASTEROIDS_WAVE_MIN + ASTEROIDS_WAVE_RATE * self.wave_count
            as f32) as u64, ASTEROIDS_WAVE_MAX);
        for _i in 0..count {
            self.spawn_asteroid();
        }
        self.wave_count += 1;
    }

    pub fn spawn_asteroid(&mut self) -> Option<usize> {
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

    pub fn destroy_asteroid(&mut self, id: usize, score_manager: &mut ScoreManager) {
        let asteroid = &mut self.asteroids[id];
        score_manager.score += (asteroid.radius * SCORE_RADIUS_FACTOR).round() as u64;
        score_manager.score += score_manager.combo as u64 * SCORE_COMBO_FACTOR;
        score_manager.increase_combo();
        asteroid.is_active = false;
        if asteroid.radius <= MIN_RADIUS {
            return;
        }
        let split_radius = asteroid.radius - DECREASE_RADIUS;
        let position = asteroid.position;
        self.create_asteroid(position, split_radius);
        self.create_asteroid(position, split_radius);
    }

    pub fn get_asteroid_at_point(&mut self, point: Vec2) -> Option<usize> {
        for i in 0..self.asteroids.len() {
            if !self.asteroids[i].is_active {
                continue
            }
            if self.asteroids[i].is_point_inside(point) {
                return Some(i)
            }
        }
        None
    }

    pub fn get_asteroid_count(&self) -> usize {
        let mut result = 0;
        for asteroid in &self.asteroids {
            if asteroid.is_active {
                result += 1;
            }
        }
        result
    }

    fn create_asteroid(&mut self, position: Vec2, radius: f32) -> Option<usize>
    {
        if self.asteroids[self.head].is_active {
            return None
        }

        let angle = rand::RandomRange::gen_range(-std::f32::consts::PI, std::f32::consts::PI);
        let speed_factor = rand::RandomRange::gen_range(MIN_SPEED_FACTOR, 1.0);
        self.asteroids[self.head] = Asteroid::new(position, radius, angle, speed_factor);
        let result = self.head;
        self.head = (self.head + 1) % MAX_COUNT;
        Some(result)
    }
}
