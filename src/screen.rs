use macroquad::math::*;

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
