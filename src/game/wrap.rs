use macroquad::math::*;

pub trait Wrap {
    fn get_wrap_position(&self) -> Vec2;
    fn set_wrap_position(&mut self, value: Vec2);
    fn get_size(&self) -> f32;

    fn handle_wrap(&mut self) {
        let mut position = self.get_wrap_position();
        let size = self.get_size();

        if position.x - size > crate::gfx::world_size().x / 2.0
            || position.x + size < -crate::gfx::world_size().x / 2.0
        {
            position.x = -position.x;
        }


        if position.y - size > crate::gfx::world_size().y / 2.0
            || position.y + size < -crate::gfx::world_size().y / 2.0
        {
            position.y = -position.y;
        }

        self.set_wrap_position(position);
    }
}
