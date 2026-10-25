struct GameState {

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

    };

    Ok(state)
}

async fn update(mut state: &GameState, delta: f32) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn draw(mut state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    macroquad::window::clear_background(macroquad::color::BLACK);
    macroquad::window::next_frame().await;
    Ok(())
}

fn terminate(mut state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    println!("Terminated game");

    Ok(())
}
