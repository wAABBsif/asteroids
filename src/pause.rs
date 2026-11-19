use macroquad::{color, input::{self, KeyCode}, math::*, shapes, text};

use crate::{scenes::{Scene, Transition}, screen::{self}};

const BACKGROUND_COLOR: color::Color = color::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.7
};

const BUTTON_UNSELECTED_COLOR: color::Color = color::Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0
};
const BUTTON_SELECTED_COLOR: color::Color = color::Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0
};
const BUTTON_TEXT_COLOR: color::Color = color::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0
};
const BUTTON_GAP: f32 = 1.5;
const BUTTON_SIZE: Vec2 = vec2(3.0, 1.0);
const BUTTON_FONT_SIZE: f32 = 0.75;
const BUTTONS: &[Button] = &[
    Button {text: "RESUME", func: || Transition::Remove }
];

pub struct PauseScene {
    selected_index: usize
}

impl PauseScene {
    pub fn new() -> PauseScene {
        PauseScene {
            selected_index: 0
        }
    }
}

impl Scene for PauseScene {
    fn update(&mut self, _delta: f32) -> Transition {
        if input::is_key_pressed(KeyCode::Escape) {
            return Transition::Remove
        }
        if input::is_key_pressed(KeyCode::W) || input::is_key_pressed(KeyCode::Up) {
            if self.selected_index == 0 {
                self.selected_index = BUTTONS.len();
            }
            self.selected_index -= 1;
        } else if input::is_key_pressed(KeyCode::S) || input::is_key_pressed(KeyCode::Down) {
            self.selected_index += 1;
            self.selected_index %= BUTTONS.len();
        }
        if input::is_key_pressed(KeyCode::Space) || input::is_key_pressed(KeyCode::Enter) {
            return (BUTTONS[self.selected_index].func)()
        }
        Transition::None
    }

    fn draw(&self) {
        let size = crate::screen::screen_size();
        shapes::draw_rectangle(0.0, 0.0, size.x, size.y, BACKGROUND_COLOR);
        let start_height = BUTTON_SIZE.y * 0.25 - BUTTONS.len() as f32 * BUTTON_GAP * 0.5;
        for i in 0..BUTTONS.len() {
            let position = vec2(BUTTON_SIZE.x * -0.5, start_height + BUTTON_GAP * (i as f32));
            let position = screen::world_to_screen_offset(position);
            let size = screen::world_to_screen(BUTTON_SIZE);
            let color = if self.selected_index == i {
                BUTTON_SELECTED_COLOR
            } else {
                BUTTON_UNSELECTED_COLOR
            };
            shapes::draw_rectangle(position.x, position.y, size.x, size.y, color);

            let font_size = BUTTON_FONT_SIZE * screen::pixels_per_unit();
            let dimensions = text::measure_text(BUTTONS[i].text, None, font_size as u16, 1.0);
            let dimensions = vec2(-dimensions.width, dimensions.height);
            let position = position + size * 0.5 + dimensions * 0.5;
            text::draw_text(BUTTONS[i].text, position.x, position.y, font_size, BUTTON_TEXT_COLOR);

        }
    }
}

struct Button {
    text: &'static str,
    func: fn() -> Transition
}
