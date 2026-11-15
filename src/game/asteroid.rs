use std::f32::consts::PI;

use macroquad::{math::*, *};

use crate::{game::{player::Player, score::ScoreManager, screen::{self, EdgeBehavior}}};
use crate::collections::SwapArray;

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
const SHAPE_RAND_FACTOR: f32 = 0.125;
const BRIGHT_COLOR: color::Color = color::Color {
    r: 0.4,
    g: 0.4,
    b: 0.5,
    a: 1.0
};
const DARK_COLOR: color::Color = color::Color {
    r: 0.3,
    g: 0.3,
    b: 0.4,
    a: 1.0
};

const MAX_POINT_COUNT: usize = (POINT_RADIUS_RATIO * SPAWN_RADIUS) as usize;
const MAX_COUNT: usize = 48;

const PARTICLE_SHAPE_RAND_FACTOR: f32 = 2.0;
const PARTICLE_RADIUS: f32 = 0.05;
const PARTICLE_MIN_SPEED: f32 = 2.0;
const PARTICLE_MAX_SPEED: f32 = 8.0;
const MAX_PARTICLE_COUNT: usize = 512;
const PARTICLE_RADIUS_SQRT_RATIO: f32 = 128.0;

pub struct AsteroidManager {
    asteroids: SwapArray<Asteroid, MAX_COUNT>,
    particles: SwapArray<Particle, MAX_PARTICLE_COUNT>,
    wave_count: u64,
}

impl AsteroidManager {
    pub fn new() -> AsteroidManager {
        AsteroidManager {
            asteroids: SwapArray::default(),
            particles: SwapArray::default(),
            wave_count: 0
        }
    }

    pub fn update(&mut self, delta: f32) {
        for a in &mut self.asteroids {
            a.update(delta);
        }
        for p in &mut self.particles {
            p.update(delta);
        }
        self.asteroids.retain(|&a| !a.position.is_nan());
        self.particles.retain(|&p| !p.position.is_nan());
        if self.asteroids.size() == 0 {
            self.initialize_wave();
        }
    }

    pub fn draw(&self, player: &Player) {
        for a in &self.asteroids {
            a.draw(player);
        }
        for p in &self.particles {
            p.draw();
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

    pub fn spawn_asteroid(&mut self) {
        let perimeter = (screen::world_size().x + screen::world_size().y) * 2.0;
        let perimeter_point = rand::gen_range(0.0, perimeter);
        let position: Vec2;
        if perimeter_point < screen::world_size().x {
            let perimeter_point = perimeter_point - screen::world_size().x * 0.5;
            position = vec2(perimeter_point, f32::INFINITY);
        } else if perimeter_point < screen::world_size().x + screen::world_size().y {
            let perimeter_point = perimeter_point - screen::world_size().x * 1.5;
            position = vec2(f32::INFINITY, perimeter_point);
        } else if perimeter_point < perimeter - screen::world_size().y {
            let perimeter_point =
                perimeter_point - screen::world_size().x * 1.5 - screen::world_size().y;
            position = vec2(perimeter_point, f32::NEG_INFINITY);
        } else {
            let perimeter_point = perimeter_point
                - screen::world_size().x * 2.0 - screen::world_size().y * 1.5;
            position = vec2(f32::NEG_INFINITY, perimeter_point);
        }

        self.create_asteroid(position, SPAWN_RADIUS);
    }

    pub fn destroy_asteroid(&mut self, id: usize, score_manager: &mut ScoreManager) {
        let asteroid = &mut self.asteroids[id];
        score_manager.score += (asteroid.radius * SCORE_RADIUS_FACTOR).round() as u64;
        score_manager.score += score_manager.combo as u64 * SCORE_COMBO_FACTOR;
        score_manager.increase_combo();
        let position = asteroid.position;
        let radius = asteroid.radius;
        asteroid.position = Vec2::NAN;
        for _i in 0..(radius.powi(2) * PARTICLE_RADIUS_SQRT_RATIO) as usize {
            self.create_particle(position, radius);
        }
        if radius <= MIN_RADIUS {
            return;
        }
        let radius = radius - DECREASE_RADIUS;
        self.create_asteroid(position, radius);
        self.create_asteroid(position, radius);
    }

    pub fn get_asteroid_at_point(&mut self, point: Vec2) -> Option<usize> {
        for i in 0..self.asteroids.size() {
            if self.asteroids[i].is_point_inside(point) {
                return Some(i)
            }
        }
        None
    }

    fn create_asteroid(&mut self, position: Vec2, radius: f32) {
        if self.asteroids.is_full() {
            return
        }

        let angle = rand::RandomRange::gen_range(-std::f32::consts::PI, std::f32::consts::PI);
        let speed_factor = rand::RandomRange::gen_range(MIN_SPEED_FACTOR, 1.0);

        self.asteroids.add(Asteroid::new(position, radius, angle, speed_factor));
    }

    fn create_particle(&mut self, asteroid_position: Vec2, asteroid_radius: f32) {
        if self.particles.is_full() {
            return
        }

        let pos_radius = rand::RandomRange::gen_range(0.0, asteroid_radius);
        let pos_angle = rand::RandomRange::gen_range(0.0, 2.0 * PI);
        let vel_angle = rand::RandomRange::gen_range(0.0, 2.0 * PI);
        let speed = rand::RandomRange::gen_range(PARTICLE_MIN_SPEED, PARTICLE_MAX_SPEED);
        let pos = asteroid_position + Vec2::from_angle(pos_angle) * pos_radius;

        self.particles.add(Particle::new(pos, vel_angle, speed));
    }
}

#[derive(Clone, Copy, Default)]
pub struct Asteroid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    shape: Polygon<MAX_POINT_COUNT>,
}

impl Asteroid {
    pub fn new(position: Vec2, radius: f32, angle: f32, speed_factor: f32) -> Asteroid {
        Asteroid {
            position: position,
            velocity: Vec2::from_angle(angle) * speed_factor * Asteroid::max_speed(radius),
            radius: radius,
            shape: Polygon::new((radius * POINT_RADIUS_RATIO) as usize, SHAPE_RAND_FACTOR / radius)
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        screen::handle_edge_behavior(self, self.position, self.radius);
    }

    pub fn draw(&self, player: &Player) {
        let pt_count = (self.radius * POINT_RADIUS_RATIO) as usize;
        for i in 0..pt_count {
            let player_dir = (player.position - self.position).normalize();
            let normal = Vec2::from_angle((i as f32) / (pt_count as f32) * 2.0 * PI);
            let dist = player.position.distance(self.position);
            let lightness = if player.is_active {
                (normal.dot(player_dir) + 1.0) * 0.5 / dist.sqrt()
            } else {
                0.0
            };
            let color = macroquad::color::Color {
                r: DARK_COLOR.r + (BRIGHT_COLOR.r - DARK_COLOR.r) * lightness,
                g: DARK_COLOR.g + (BRIGHT_COLOR.g - DARK_COLOR.g) * lightness,
                b: DARK_COLOR.b + (BRIGHT_COLOR.b - DARK_COLOR.b) * lightness,
                a: 1.0
            };
            self.shape.draw_triangle(self.position, self.radius, pt_count, i, color);
        }
    }

    pub fn is_point_inside(&self, point: Vec2) -> bool {
        return self.position.distance_squared(point) <= self.radius * self.radius
    }

    fn max_speed(radius: f32) -> f32 {
        MAX_SPEED_CONSTANT + MAX_SPEED_COEFFICIENT / radius
    }
}

impl EdgeBehavior for Asteroid {
    fn on_hit_screen_edge(&mut self, axis: screen::Axis) {
        screen::screen_wrap(axis, &mut self.position, self.radius);
    }
}

#[derive(Clone, Copy, Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    shape: Polygon<2>
}

impl Particle {
    pub fn new(position: Vec2, angle: f32, speed: f32) -> Particle {
        Particle {
            position: position,
            velocity: Vec2::from_angle(angle) * speed,
            shape: Polygon::new(2, PARTICLE_SHAPE_RAND_FACTOR),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        screen::handle_edge_behavior(self, self.position, PARTICLE_RADIUS * PARTICLE_SHAPE_RAND_FACTOR);
    }

    pub fn draw(&self) {
        self.shape.draw_triangle(self.position, PARTICLE_RADIUS, 3, 0, DARK_COLOR);
    }
}

impl EdgeBehavior for Particle {
    fn on_hit_screen_edge(&mut self, axis: screen::Axis) {
        self.position = Vec2::NAN;
    }
}


#[derive(Clone, Copy)]
struct Polygon<const C: usize> {
    radii: [f32; C],
}

impl<const C: usize> Polygon<C> {
    fn new(count: usize, rand_factor: f32) -> Polygon<C> {
        let mut result = Polygon {
            radii: [0.0; C]
        };
        for radius in &mut result.radii[0..count] {
            *radius = 1.0 + rand::RandomRange::gen_range(-rand_factor, rand_factor);
        }

        result
    }

    fn draw_triangle(&self, position: Vec2, radius: f32, count: usize, index: usize, 
        color: macroquad::color::Color)
    {
        let pos = screen::world_to_screen_offset(position);
        let angles = [
            (index as f32) / (count as f32) * 2.0 * PI,
            ((index + 1) as f32) / (count as f32) * 2.0 * PI
        ];
        let radii = [
            self.radii[index] * radius * screen::pixels_per_unit(),
            self.radii[(index + 1) % count] * radius * screen::pixels_per_unit()
        ];
        let triangle_pts = [
            pos,
            pos + Vec2::from_angle(angles[0]) * radii[0],
            pos + Vec2::from_angle(angles[1]) * radii[1]
        ];
        shapes::draw_triangle(triangle_pts[0], triangle_pts[1], triangle_pts[2], color);
    }
}

impl<const C: usize> Default for Polygon<C> {
    fn default() -> Self {
        Self {
            radii: [0.0; C]
        }
    }
}
