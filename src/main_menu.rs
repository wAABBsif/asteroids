use macroquad::{color, math::*};

use crate::{game, menus::*, scenes::{Scene, Transition}, screen::{self, Background}};

const TITLE: &str = "Asteroids";
const TITLE_FONT_SIZE: f32 = 1.5;
const TITLE_HEIGHT: f32 = -1.5;
const TITLE_COLOR: color::Color = macroquad::color::WHITE;

const MENU: Menu<2> = Menu {
    unselected_color: color::WHITE,
    selected_color: color::YELLOW,
    text_color: color::BLACK,
    position_offset: vec2(0.0, 1.5),
    button_gap: 1.5,
    button_size: vec2(4.0, 1.0),
    font_size: 0.75,
    buttons: [
        Button { text: "PLAY", func: || Transition::Clear(Box::new(game::GameScene::new())) },
        Button { text: "QUIT", func: || Transition::Exit }
    ]
};

pub struct MainMenuScene {
    background: Background,
    selected_index: usize
}

impl MainMenuScene {
    pub fn new() -> MainMenuScene {
        MainMenuScene {
            background: Background::new(),
            selected_index: 0
        }
    }
}

impl Scene for MainMenuScene {
    fn update(&mut self, _delta: f32) -> Transition {
        return MENU.update(&mut self.selected_index);
    }

    fn draw(&self) {
        let font_size = TITLE_FONT_SIZE * screen::pixels_per_unit();
        let dimensions = macroquad::text::measure_text(TITLE, None, font_size as u16, 1.0);
        let dimenions = vec2(-dimensions.width, dimensions.height);
        let position = screen::world_to_screen_offset(vec2(0.0, TITLE_HEIGHT)) + dimenions * 0.5;
        macroquad::text::draw_text(TITLE, position.x, position.y, font_size, TITLE_COLOR);
        self.background.draw();
        MENU.draw(self.selected_index);
    }
}
