use macroquad::math::*;

const WORLD_WIDTH: f32 = 16.0;

pub fn screen() -> Vec2 {
    if cfg!(test) {
        return vec2(16.0, 9.0);
    }
    vec2(macroquad::window::screen_width(), macroquad::window::screen_height())
}

pub fn world_size() -> Vec2 {
    vec2(WORLD_WIDTH, WORLD_WIDTH / aspect_ratio())
}

pub fn aspect_ratio() -> f32 {
    screen().x / screen().y
}

pub fn pixels_per_unit() -> f32 {
    screen().x / world_size().x
}

pub fn world_to_screen(world_coords: Vec2) -> Vec2 {
    world_coords / world_size() * screen()
}

pub fn screen_to_world(screen_coords: Vec2) -> Vec2 {
    screen_coords / screen() * world_size()
}

pub fn world_to_screen_offset(world_coords: Vec2) -> Vec2 {
    world_to_screen(world_coords) + screen() / vec2(2.0, 2.0)
}

pub fn screen_to_world_offset(screen_coords: Vec2) -> Vec2 {
    screen_to_world(screen_coords) - world_size() / vec2(2.0, 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_1() {
        assert!(screen_to_world(world_to_screen(vec2(0.0, 0.0)))
            .distance_squared(vec2(0.0, 0.0)) < 0.01);
    }

    #[test]
    fn test_reverse_2() {
        assert!(screen_to_world(world_to_screen(vec2(1.0, -0.5)))
            .distance_squared(vec2(1.0, -0.5)) < 0.01);
    }

    #[test]
    fn test_reverse_3() {
        assert!(screen_to_world(world_to_screen(vec2(-0.3, 2.0)))
            .distance_squared(vec2(-0.3, 2.0)) < 0.01);
    }

    #[test]
    fn test_reverse_offset_1() {
        assert!(screen_to_world_offset(world_to_screen_offset(vec2(0.0, 0.0)))
            .distance_squared(vec2(0.0, 0.0)) < 0.01);
    }

    #[test]
    fn test_reverse_offset_2() {
        assert!(screen_to_world_offset(world_to_screen_offset(vec2(1.0, -0.5)))
            .distance_squared(vec2(1.0, -0.5)) < 0.01);
    }

    #[test]
    fn test_reverse_offset_3() {
        assert!(screen_to_world_offset(world_to_screen_offset(vec2(-0.3, 2.0)))
            .distance_squared(vec2(-0.3, 2.0)) < 0.01);
    }
}
