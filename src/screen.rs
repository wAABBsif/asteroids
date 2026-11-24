use macroquad::{color, math::*};

const MIN_WORLD_WIDTH: f32 = 16.0;
const MIN_WORLD_HEIGHT: f32 = 9.0;

const STAR_COUNT: usize = 64;
const STAR_RANGE: f32 = 1.25;
const STAR_RADIUS: f32 = 0.02;
const STAR_COLOR: color::Color = color::Color {
    r: 0.9,
    g: 0.9,
    b: 0.3,
    a: 1.0
};

pub fn screen_size() -> Vec2 {
    vec2(macroquad::window::screen_width(), macroquad::window::screen_height())
}

pub fn world_size() -> Vec2 {
    let aspect = aspect_ratio();
    if MIN_WORLD_WIDTH / aspect <= MIN_WORLD_HEIGHT {
        vec2(MIN_WORLD_HEIGHT * aspect, MIN_WORLD_HEIGHT)
    } else {
        vec2(MIN_WORLD_WIDTH, MIN_WORLD_WIDTH / aspect)
    }
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

pub struct Background {
    stars: [Vec2; STAR_COUNT]
}

impl Background {
    pub fn new() -> Background {
        Background {
            stars: std::array::from_fn(|_i| vec2(
                macroquad::rand::gen_range(0.5 - STAR_RANGE * 0.5, 0.5 + STAR_RANGE * 0.5),
                macroquad::rand::gen_range(0.5 - STAR_RANGE * 0.5, 0.5 + STAR_RANGE * 0.5)
            ))
        }
    }

    pub fn draw(&self) {
        for star in &self.stars {
            let pos = screen_size() * *star;
            macroquad::shapes::draw_circle(pos.x, pos.y, STAR_RADIUS * pixels_per_unit(), STAR_COLOR);
        }
    }
}
