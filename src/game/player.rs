use macroquad::{math::*, *};

use crate::gfx::pixels_per_unit;

const WIDTH: f32 = 0.2;
const HEIGHT: f32 = 0.3;
const COLOR: macroquad::color::Color = macroquad::color::WHITE;

const TRAIL_LENGTH: usize = 64;
const TRAIL_INTERVAL: f32 = 0.01;
const TRAIL_MAX_RADIUS: f32 = 0.075;
const TRAIL_RADIUS_EXP: f32 = 0.5;
const TRAIL_COLOR: macroquad::color::Color = macroquad::color::ORANGE;
const TRAIL_ALPHA_EXP: f32 = 3.0;

const ACCELERATION: f32 = 8.0;
const MAX_SPEED: f32 = 16.0;
const TURN_SPEED: f32 = 6.0;
const DRAG: f32 = 0.8;

pub struct Player {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    angular_velocity: f32,
    trail_timer: f32,
    trail_pts: [Vec2; TRAIL_LENGTH],
    is_accelerating: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            angle:  0.0,
            angular_velocity: 0.0,
            trail_timer: 0.0,
            trail_pts: [Vec2::ZERO; TRAIL_LENGTH],
            is_accelerating: false
        }
    }

    pub fn update(&mut self, delta: f32) {
        if input::is_key_down(input::KeyCode::W) || input::is_key_down(input::KeyCode::Up) {
            self.is_accelerating = true;
            self.velocity += self.up_direction() * ACCELERATION * delta;
        } else {
            self.is_accelerating = false;
        }
        self.velocity *= (1.0 - DRAG).powf(delta);
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);

        self.angular_velocity = 0.0;
        if input::is_key_down(input::KeyCode::D) || input::is_key_down(input::KeyCode::Right) {
            self.angular_velocity += TURN_SPEED;
        }
        if input::is_key_down(input::KeyCode::A) || input::is_key_down(input::KeyCode::Left) {
            self.angular_velocity -= TURN_SPEED;
        }

        self.position += self.velocity * delta;
        self.angle += self.angular_velocity * delta;

        self.update_trail(delta);
    }

    fn update_trail(&mut self, delta: f32) {
        self.trail_timer -= delta;
        while self.trail_timer <= 0.0 {
            for i in (1..TRAIL_LENGTH).rev() {
                self.trail_pts[i] = self.trail_pts[i - 1];
            }
            self.trail_pts[0] = if self.is_accelerating {
                self.position - self.up_direction() * HEIGHT - self.velocity * -self.trail_timer
            } else {
                vec2(f32::NAN, f32::NAN)
            };
            self.trail_timer += TRAIL_INTERVAL;
        }
    }

    pub fn draw(&self) {
        self.draw_trail();

        let points = [
            self.position + self.up_direction() * HEIGHT,
            self.position - self.up_direction() * HEIGHT + self.right_direction() * WIDTH,
            self.position - self.up_direction() * HEIGHT - self.right_direction() * WIDTH,
        ];

        shapes::draw_triangle(
            crate::gfx::world_to_screen_pos(points[0]),
            crate::gfx::world_to_screen_pos(points[1]),
            crate::gfx::world_to_screen_pos(points[2]),
            COLOR
        );
    }

    fn draw_trail(&self) {
        for i in 0..TRAIL_LENGTH-1 {
            let point = crate::gfx::world_to_screen_pos(self.trail_pts[i]);
            let point_frac = (TRAIL_LENGTH - i) as f32 / TRAIL_LENGTH as f32;
            let color = TRAIL_COLOR.with_alpha(point_frac.powf(TRAIL_ALPHA_EXP));
            let radius = TRAIL_MAX_RADIUS * point_frac.powf(TRAIL_RADIUS_EXP) * pixels_per_unit();

            shapes::draw_circle(point.x, point.y, radius, color);
        }
    }

    fn right_direction(&self) -> Vec2 {
        vec2(1.0, 0.0).rotate(Vec2::from_angle(self.angle))
    }

    fn up_direction(&self) -> Vec2 {
        vec2(0.0, -1.0).rotate(Vec2::from_angle(self.angle))
    }
}
