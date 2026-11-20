mod player;
mod bullet;
mod asteroid;
mod world;
mod score;
mod hud;

use macroquad::math::*;

use player::Player;
use bullet::BulletManager;
use asteroid::AsteroidManager;
use score::ScoreManager;
use hud::HUD;
use world::Background;

use crate::{scenes::{Scene, Transition}};

pub struct GameScene {
    pub background: Background,
    pub player: Player,
    pub bullet_manager: BulletManager,
    pub asteroid_manager: AsteroidManager,
    pub score_manager: ScoreManager,
    pub hud: HUD,
}

impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            background: Background::new(),
            player: Player::new(),
            bullet_manager: BulletManager::new(),
            asteroid_manager: AsteroidManager::new(),
            score_manager: ScoreManager::new(),
            hud: HUD::new(),
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, delta: f32) -> Transition {
        self.player.update(delta, &mut self.bullet_manager, &mut self.asteroid_manager);
        self.bullet_manager.update(delta, &mut self.asteroid_manager, &mut self.score_manager);
        self.asteroid_manager.update(delta);
        self.score_manager.update(delta);
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Escape) {
            return Transition::Add(Box::new(crate::pause::PauseScene::new()))
        }
        Transition::None
    }

    fn draw(&self) {
        self.background.draw();
        self.bullet_manager.draw();
        self.asteroid_manager.draw(&self.player);
        self.player.draw();
        self.hud.draw(&self.score_manager, &self.player);
    }
}
