pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut scenes = initialize()?;
    while !macroquad::input::is_quit_requested() {
        update(&mut scenes, macroquad::time::get_frame_time()).await?;
        draw(&mut scenes).await?;
    }
    terminate(&mut scenes)?;

    Ok(())
}

fn initialize() -> Result<Vec<Box<dyn Scene>>, Box<dyn std::error::Error>> {
    println!("Initializing Asteroids");

    //handle program termination manually
    println!("Disabling macroquad auto-termination");
    macroquad::input::prevent_quit();

    println!("Initializing game state");
    let scenes: Vec<Box<dyn Scene>> = vec![Box::new(crate::game::GameState::new())];

    Ok(scenes)
}

async fn update(scenes: &mut Vec<Box<dyn Scene>>, delta: f32)
    -> Result<(), Box<dyn std::error::Error>>
{
    let scene = scenes.last_mut();
    if scene.is_none() {
        return Ok(())
    }
    let scene = scene.unwrap();
    match scene.update(delta) {
        Transition::None => {},
        Transition::Remove => {
            scenes.pop();
        },
        Transition::Add(s) => {
            scenes.push(s);
        }
    };
    Ok(())
}

async fn draw(scenes: &mut Vec<Box<dyn Scene>>) -> Result<(), Box<dyn std::error::Error>> {
    macroquad::window::clear_background(macroquad::color::BLACK);
    for scene in scenes {
        scene.draw();
    }
    macroquad::window::next_frame().await;
    Ok(())
}

fn terminate(mut _scenes: &mut Vec<Box<dyn Scene>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Terminated game");

    Ok(())
}

pub enum Transition {
    None,
    Remove,
    Add(Box<dyn Scene>),
}

pub trait Scene {
    fn update(&mut self, delta: f32) -> Transition;
    fn draw(&self);
}
