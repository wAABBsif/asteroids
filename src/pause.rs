use macroquad::{color, input::{self, KeyCode}, math::*, shapes};

use crate::{menus::*, main_menu, game, scenes::{Scene, Transition}};

const BACKGROUND_COLOR: color::Color = color::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.7
};

const MENU: Menu<3> = Menu {
    unselected_color: color::Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0
    },
    selected_color: color::Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0
    },
    text_color: color::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0
    },
    position_offset: vec2(0.0, 0.0),
    button_gap: 1.5,
    button_size: vec2(4.0, 1.0),
    font_size: 0.75,
    buttons: [
        Button { text: "RESUME", func: || Transition::Remove },
        Button { text: "RESTART", func: || Transition::Clear(Box::new(game::GameScene::new())) },
        Button { text: "MAIN MENU", func: || Transition::Clear(Box::new(main_menu::MainMenuScene::new())) }
    ]
};

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
        return MENU.update(&mut self.selected_index);
    }

    fn draw(&self) {
        let size = crate::screen::screen_size();
        shapes::draw_rectangle(0.0, 0.0, size.x, size.y, BACKGROUND_COLOR);
        MENU.draw(self.selected_index);
    }
}
