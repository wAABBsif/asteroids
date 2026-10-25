use macroquad::{color::*, input::*, window::*};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    initialize()?;
    while !macroquad::input::is_quit_requested() {
        update().await?;
    }
    terminate()?;

    Ok(())
}

fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing game");

    //handle program termination manually
    prevent_quit();

    Ok(())
}

async fn update() -> Result<(), Box<dyn std::error::Error>> {
    clear_background(BLACK);
    next_frame().await;

    Ok(())
}

fn terminate() -> Result<(), Box<dyn std::error::Error>> {
    println!("Terminated game");

    Ok(())
}
