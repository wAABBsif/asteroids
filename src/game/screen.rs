use macroquad::math::*;

use crate::game::screen;

const WORLD_WIDTH: f32 = 16.0;

pub fn screen_size() -> Vec2 {
    if cfg!(test) {
        return vec2(16.0, 9.0);
    }
    vec2(macroquad::window::screen_width(), macroquad::window::screen_height())
}

pub fn world_size() -> Vec2 {
    vec2(WORLD_WIDTH, WORLD_WIDTH / aspect_ratio())
}

pub fn aspect_ratio() -> f32 {
    screen_size().x / screen_size().y
}

pub fn pixels_per_unit() -> f32 {
    screen_size().x / world_size().x
}

pub fn world_to_screen(world_coords: Vec2) -> Vec2 {
    world_coords / world_size() * screen_size()
}

pub fn world_to_screen_offset(world_coords: Vec2) -> Vec2 {
    world_to_screen(world_coords) + screen_size() / vec2(2.0, 2.0)
}

pub enum Axis {
    X,
    Y
}

pub trait EdgeBehavior {
    fn on_hit_screen_edge(&mut self, axis: Axis);
}

pub fn screen_wrap(axis: Axis, position: &mut Vec2, size: f32) {
    let bounds = screen::world_size() * 0.5 + Vec2::splat(size);
    match axis {
        Axis::X => position.x = -position.x.clamp(-bounds.x, bounds.x),
        Axis::Y => position.y = -position.y.clamp(-bounds.y, bounds.y),
    }
}

pub fn handle_edge_behavior(obj: &mut impl EdgeBehavior, position: Vec2, size: f32) {
    let bounds = screen::world_size() * 0.5 + Vec2::splat(size);

    if position.x < -bounds.x || position.x > bounds.x {
        obj.on_hit_screen_edge(Axis::X);
    }

    if position.y < -bounds.y || position.y > bounds.y {
        obj.on_hit_screen_edge(Axis::Y);
    }
}
