use macroquad::math::*;

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
