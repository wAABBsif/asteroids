mod player;
mod bullet;
mod asteroid;
mod wrap;
mod score;
mod hud;

use macroquad::math::*;

use player::Player;
use bullet::BulletManager;
use asteroid::AsteroidManager;
use score::ScoreManager;
use hud::HUD;

struct GameState {
    pub player: Player,
    pub bullet_manager: BulletManager,
    pub asteroid_manager: AsteroidManager,
    pub score_manager: ScoreManager,
    pub hud: HUD,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = initialize()?;
    while !macroquad::input::is_quit_requested() {
        update(&mut state, macroquad::time::get_frame_time()).await?;
        draw(&mut state).await?;
    }
    terminate(&mut state)?;

    Ok(())
}

fn initialize() -> Result<GameState, Box<dyn std::error::Error>> {
    println!("Initializing game");

    //handle program termination manually
    println!("Disabling macroquad auto-termination");
    macroquad::input::prevent_quit();

    println!("Initializing game state");
    let state = GameState {
        player: Player::new(),
        bullet_manager: BulletManager::new(),
        asteroid_manager: AsteroidManager::new(),
        score_manager: ScoreManager::new(),
        hud: HUD::new(),
    };

    Ok(state)
}

async fn update(state: &mut GameState, delta: f32) -> Result<(), Box<dyn std::error::Error>> {
    state.player.update(delta, &mut state.bullet_manager, &mut state.asteroid_manager);
    state.bullet_manager.update(delta, &mut state.asteroid_manager, &mut state.score_manager);
    state.asteroid_manager.update(delta);
    state.score_manager.update(delta);

    Ok(())
}

async fn draw(state: &mut GameState) -> Result<(), Box<dyn std::error::Error>> {
    macroquad::window::clear_background(macroquad::color::BLACK);
    state.bullet_manager.draw();
    state.player.draw();
    state.asteroid_manager.draw(&state.player);
    state.hud.draw(&state.score_manager);
    macroquad::window::next_frame().await;
    Ok(())
}

fn terminate(mut state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    println!("Terminated game");

    Ok(())
}
