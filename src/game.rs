mod player;
mod bullet;
mod asteroid;
mod screen;
mod score;
mod hud;

use macroquad::math::*;

use player::Player;
use bullet::BulletManager;
use asteroid::AsteroidManager;
use score::ScoreManager;
use hud::HUD;

use crate::scenes::{Transition, Scene};

pub struct GameState {
    pub player: Player,
    pub bullet_manager: BulletManager,
    pub asteroid_manager: AsteroidManager,
    pub score_manager: ScoreManager,
    pub hud: HUD,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: Player::new(),
            bullet_manager: BulletManager::new(),
            asteroid_manager: AsteroidManager::new(),
            score_manager: ScoreManager::new(),
            hud: HUD::new(),
        }
    }
}

impl Scene for GameState {
    fn update(&mut self, delta: f32) -> Transition {
        self.player.update(delta, &mut self.bullet_manager, &mut self.asteroid_manager);
        self.bullet_manager.update(delta, &mut self.asteroid_manager, &mut self.score_manager);
        self.asteroid_manager.update(delta);
        self.score_manager.update(delta);
        Transition::None
    }

    fn draw(&self) {
        self.bullet_manager.draw();
        self.asteroid_manager.draw(&self.player);
        self.player.draw();
        self.hud.draw(&self.score_manager, &self.player);
    }
}
