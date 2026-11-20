use macroquad::{math::*, *};

use crate::screen;

const STAR_COUNT: usize = 64;
const STAR_RANGE: f32 = 1.25;
const STAR_RADIUS: f32 = 0.02;
const STAR_COLOR: color::Color = color::Color {
    r: 0.9,
    g: 0.9,
    b: 0.3,
    a: 1.0
};

pub enum Axis {
    X,
    Y
}

pub trait EdgeBehavior {
    fn on_hit_screen_edge(&mut self, axis: Axis);
}

pub fn screen_wrap(axis: Axis, position: &mut Vec2, size: f32) {
    let bounds = crate::screen::world_size() * 0.5 + Vec2::splat(size);
    match axis {
        Axis::X => position.x = -position.x.clamp(-bounds.x, bounds.x),
        Axis::Y => position.y = -position.y.clamp(-bounds.y, bounds.y),
    }
}

pub fn handle_edge_behavior(obj: &mut impl EdgeBehavior, position: Vec2, size: f32) {
    let bounds = crate::screen::world_size() * 0.5 + Vec2::splat(size);

    if position.x < -bounds.x || position.x > bounds.x {
        obj.on_hit_screen_edge(Axis::X);
    }

    if position.y < -bounds.y || position.y > bounds.y {
        obj.on_hit_screen_edge(Axis::Y);
    }
}

pub struct Background {
    stars: [Vec2; STAR_COUNT]
}

impl Background {
    pub fn new() -> Background {
        Background {
            stars: std::array::from_fn(|_i| vec2(
                rand::gen_range(0.5 - STAR_RANGE * 0.5, 0.5 + STAR_RANGE * 0.5),
                rand::gen_range(0.5 - STAR_RANGE * 0.5, 0.5 + STAR_RANGE * 0.5)
            ))
        }
    }

    pub fn draw(&self) {
        for star in &self.stars {
            let pos = screen::screen_size() * *star;
            shapes::draw_circle(pos.x, pos.y, STAR_RADIUS * screen::pixels_per_unit(), STAR_COLOR);
        }
    }
}
