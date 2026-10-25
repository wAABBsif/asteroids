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
    macroquad::input::prevent_quit();

    Ok(())
}

async fn update() -> Result<(), Box<dyn std::error::Error>> {
    macroquad::window::clear_background(macroquad::color::BLACK);
    macroquad::window::next_frame().await;
    Ok(())
}

fn terminate() -> Result<(), Box<dyn std::error::Error>> {
    println!("Terminated game");

    Ok(())
}
