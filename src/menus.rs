use macroquad::{color, input::{self, KeyCode}, math::*, shapes, text};
use crate::{screen, scenes};

pub struct Menu<const N: usize> {
    pub unselected_color: color::Color,
    pub selected_color: color::Color,
    pub text_color: color::Color,
    pub button_gap: f32,
    pub button_size: Vec2,
    pub font_size: f32,
    pub buttons: [Button; N]
}

impl<const N: usize> Menu<N> {
    pub fn update(&self, selected_index: &mut usize) -> scenes::Transition {
        if input::is_key_pressed(KeyCode::W) || input::is_key_pressed(KeyCode::Up) {
            if *selected_index == 0 {
                *selected_index = self.buttons.len();
            }
            *selected_index -= 1;
        } else if input::is_key_pressed(KeyCode::S) || input::is_key_pressed(KeyCode::Down) {
            *selected_index += 1;
            *selected_index %= self.buttons.len();
        }
        if input::is_key_pressed(KeyCode::Space) || input::is_key_pressed(KeyCode::Enter) {
            return (self.buttons[*selected_index].func)()
        }
        scenes::Transition::None
    }

    pub fn draw(&self, selected_index: usize) {
        let start_height = self.button_size.y * 0.25 - self.buttons.len() as f32 * self.button_gap * 0.5;
        for i in 0..self.buttons.len() {
            let position = vec2(self.button_size.x * -0.5, start_height + self.button_gap * (i as f32));
            let position = screen::world_to_screen_offset(position);
            let size = screen::world_to_screen(self.button_size);
            let color = if selected_index == i {
                self.selected_color
            } else {
                self.unselected_color
            };
            shapes::draw_rectangle(position.x, position.y, size.x, size.y, color);

            let font_size = self.font_size * screen::pixels_per_unit();
            let dimensions = text::measure_text(self.buttons[i].text, None, font_size as u16, 1.0);
            let dimensions = vec2(-dimensions.width, dimensions.height);
            let position = position + size * 0.5 + dimensions * 0.5;
            text::draw_text(self.buttons[i].text, position.x, position.y, font_size, self.text_color);
        }
    }
}

pub struct Button {
    pub text: &'static str,
    pub func: fn() -> scenes::Transition
}
