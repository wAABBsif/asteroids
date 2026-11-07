use macroquad::math::*;

use crate::game::score::ScoreManager;

const FONT_SIZE: f32 = 0.75;
const FONT_COLOR: macroquad::color::Color = macroquad::color::WHITE;
const FONT_ALPHA_EXP: f32 = 0.5;

const SCORE_TEXT_POSITION: Vec2 = vec2(0.2, 0.6);

const COMBO_TEXT_POSITION: Vec2 = vec2(0.2, 1.25);

pub struct HUD {
    pub is_visible: bool
}

impl HUD {
    pub fn new() -> HUD {
        HUD {
            is_visible: true
        }
    }

    pub fn draw(&mut self, score_manager: &ScoreManager) {
        if !self.is_visible {
            return;
        }
        let size = crate::gfx::pixels_per_unit() * FONT_SIZE;

        let point = crate::gfx::world_to_screen(SCORE_TEXT_POSITION);
        let text = format!("SCORE {}", score_manager.score);
        macroquad::text::draw_text(&text, point.x, point.y, size, FONT_COLOR);

        let point = crate::gfx::world_to_screen(COMBO_TEXT_POSITION);
        let text = format!("     x{}", score_manager.combo);
        let color = FONT_COLOR.with_alpha(score_manager.get_combo_timer_frac().powf(
                FONT_ALPHA_EXP));

        macroquad::text::draw_text(&text, point.x, point.y, size, color);
    }
}
