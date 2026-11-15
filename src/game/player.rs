use std::f32::consts::PI;

use macroquad::{math::*, *};

use crate::collections::CircleArray;
use crate::game::{asteroid::AsteroidManager, bullet::BulletManager, screen::{self, EdgeBehavior}};

const WIDTH: f32 = 0.2;
const HEIGHT: f32 = 0.3;
const COLOR: color::Color = color::WHITE;

const TRAIL_LENGTH: usize = 64;
const TRAIL_INTERVAL: f32 = 0.01;
const TRAIL_MAX_RADIUS: f32 = 0.075;
const TRAIL_RADIUS_EXP: f32 = 0.5;
const TRAIL_COLOR: color::Color = color::ORANGE;
const TRAIL_ALPHA_EXP: f32 = 3.0;

const ACCELERATION: f32 = 8.0;
const TURN_SPEED: f32 = 6.0;
const DRAG: f32 = 0.8;

const INITIAL_LIVES: u16 = 3;
const RESPAWN_TIME: f32 = 2.0;
const INVINCIBILITY_TIME: f32 = 1.0;
const INVINCIBILITY_DISAPPEAR_TIME: f32 = 0.2;

pub struct Player {
    pub is_active: bool,
    pub lives: u16,
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub angular_velocity: f32,
    is_accelerating: bool,
    timer: f32,
    trail: Trail
}

impl Player {
    pub fn new() -> Player {
        Player {
            is_active: true,
            lives: INITIAL_LIVES,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            angle:  -PI / 2.0,
            angular_velocity: 0.0,
            is_accelerating: false,
            timer: 0.0,
            trail: Trail {
                pts: CircleArray::default(),
                timer: 0.0,
            },
        }
    }

    pub fn kill(&mut self) {
        self.timer = RESPAWN_TIME;
        self.is_active = false;
    }

    pub fn respawn(&mut self) {
        let lives = self.lives - 1;
        *self = Player::new();
        self.timer = INVINCIBILITY_TIME;
        self.lives = lives;
    }

    pub fn update(&mut self, delta: f32, bullet_manager: &mut BulletManager,
        asteroid_manager: &mut AsteroidManager)
    {
        if self.is_active {
            self.handle_active(delta, bullet_manager, asteroid_manager);
        } else {
            self.handle_inactive(delta);
        }
        self.trail.update(delta, self.position, self.up_direction(), self.velocity,
            self.is_accelerating);
    }

    fn handle_active(&mut self, delta: f32, bullet_manager: &mut BulletManager,
        asteroid_manager: &mut AsteroidManager)
    {
        self.timer -= delta;
        self.handle_rotation();
        self.handle_movement(delta);
        self.update_transform(delta);
        self.handle_shooting(bullet_manager);
        self.handle_collision(asteroid_manager);
    }

    fn handle_inactive(&mut self, delta: f32) {
        self.timer -= delta;
        if self.timer <= 0.0 && self.lives > 0 {
            self.respawn();
            return;
        }
        self.position = Vec2::ZERO;
        self.is_accelerating = false;
    }

    fn handle_rotation(&mut self) {
        self.angular_velocity = 0.0;
        if input::is_key_down(input::KeyCode::D) || input::is_key_down(input::KeyCode::Right) {
            self.angular_velocity += TURN_SPEED;
        }
        if input::is_key_down(input::KeyCode::A) || input::is_key_down(input::KeyCode::Left) {
            self.angular_velocity -= TURN_SPEED;
        }
    }

    fn handle_movement(&mut self, delta: f32) {
        if input::is_key_down(input::KeyCode::W) || input::is_key_down(input::KeyCode::Up) {
            self.is_accelerating = true;
            self.velocity += self.up_direction() * ACCELERATION * delta;
        } else {
            self.is_accelerating = false;
        }
        self.velocity *= (1.0 - DRAG).powf(delta);
    }

    fn update_transform(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        self.angle += self.angular_velocity * delta;
        screen::handle_edge_behavior(self, self.position, HEIGHT.max(WIDTH));
    }

    fn handle_shooting(&self, bullet_manager: &mut BulletManager) {
        if input::is_key_pressed(input::KeyCode::Space)
            || input::is_mouse_button_pressed(input::MouseButton::Left)
        {
            let fire_point = self.position + self.up_direction() * HEIGHT;
            bullet_manager.create_bullet(fire_point, self.angle);
        }
    }

    fn handle_collision(&mut self, asteroid_manager: &mut AsteroidManager) {
        let colliding = asteroid_manager.get_asteroid_at_point(self.position);
        if colliding.is_some() && self.timer <= 0.0 {
            self.kill();
        }
    }

    pub fn draw(&self) {
        self.trail.draw();

        if !self.is_active {
            return;
        }

        if self.timer >= 0.0 {
            if self.timer % INVINCIBILITY_DISAPPEAR_TIME * 2.0 < INVINCIBILITY_DISAPPEAR_TIME {
                return;
            }
        }

        Player::draw_player(self.position, self.angle, 1.0, true);
    }

    pub fn draw_player(position: Vec2, angle: f32, scale: f32, offset: bool) {
        let right = vec2(0.0, -1.0).rotate(Vec2::from_angle(angle));
        let up = vec2(1.0, 0.0).rotate(Vec2::from_angle(angle));
        let height = HEIGHT * scale;
        let width = WIDTH * scale;
        let points = [
            position + up * height,
            position - up * height + right * width,
            position - up * height - right * width,
        ];

        if offset {
            shapes::draw_triangle(
                screen::world_to_screen_offset(points[0]),
                screen::world_to_screen_offset(points[1]),
                screen::world_to_screen_offset(points[2]),
                COLOR
            );
        } else {
            shapes::draw_triangle(
                screen::world_to_screen(points[0]),
                screen::world_to_screen(points[1]),
                screen::world_to_screen(points[2]),
                COLOR
            );
        }
    }

    fn up_direction(&self) -> Vec2 {
        vec2(1.0, 0.0).rotate(Vec2::from_angle(self.angle))
    }
}

impl EdgeBehavior for Player {
    fn on_hit_screen_edge(&mut self, axis: screen::Axis) {
        screen::screen_wrap(axis, &mut self.position, HEIGHT.max(WIDTH));
    }
}

struct Trail {
    pts: CircleArray<Vec2, TRAIL_LENGTH>,
    timer: f32
}

impl Trail {
    fn update(&mut self, delta: f32, position: Vec2, up_dir: Vec2, velocity: Vec2,
        is_accelerating: bool)
    {
        self.timer -= delta;
        while self.timer <= 0.0 {
            self.pts.add(if is_accelerating {
                position - up_dir * HEIGHT + velocity * self.timer
            } else {
                Vec2::NAN
            });
            self.timer += TRAIL_INTERVAL;
        }
    }

    fn draw(&self) {
        for (i, pt) in (&self.pts).into_iter().enumerate() {
            let point = screen::world_to_screen_offset(*pt);
            let point_frac = (i) as f32 / TRAIL_LENGTH as f32;
            let color = TRAIL_COLOR.with_alpha(point_frac.powf(TRAIL_ALPHA_EXP));
            let radius = TRAIL_MAX_RADIUS * point_frac.powf(TRAIL_RADIUS_EXP)
                * screen::pixels_per_unit();
            shapes::draw_circle(point.x, point.y, radius, color);
        }
    }
}
