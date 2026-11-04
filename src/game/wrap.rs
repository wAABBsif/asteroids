use macroquad::math::*;

pub trait Wrap {
    fn get_wrap_position(&self) -> Vec2;
    fn set_wrap_position(&mut self, value: Vec2);
    fn get_size(&self) -> f32;

    fn handle_wrap(&mut self) {
        let mut position = self.get_wrap_position();
        let size = self.get_size();
        let world_radius = crate::gfx::world_size() * 0.5;
        let bounds = vec2(world_radius.x + size, world_radius.y + size);

        if position.x < -bounds.x || position.x > bounds.x {
            position.x = -position.x;
        }

        if position.y < -bounds.y || position.y > bounds.y {
            position.y = -position.y;
        }

        position = position.clamp(-bounds, bounds);
        self.set_wrap_position(position);
    }
}
