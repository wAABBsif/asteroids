mod player;

use player::Player;

struct GameState {
    pub player: Player,
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
    macroquad::input::prevent_quit();

    let state = GameState {
        player: Player::new(),
    };

    Ok(state)
}

async fn update(state: &mut GameState, delta: f32) -> Result<(), Box<dyn std::error::Error>> {
    state.player.update(delta);

    Ok(())
}

async fn draw(state: &mut GameState) -> Result<(), Box<dyn std::error::Error>> {
    macroquad::window::clear_background(macroquad::color::BLACK);
    state.player.draw();
    macroquad::window::next_frame().await;

    Ok(())
}

fn terminate(mut state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    println!("Terminated game");

    Ok(())
}
