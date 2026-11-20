pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut scenes = initialize()?;
    let mut is_quit_requested = false;
    while !macroquad::input::is_quit_requested() && !is_quit_requested {
        is_quit_requested = update(&mut scenes, macroquad::time::get_frame_time()).await?;
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

    println!("Seeding RNG");
    let now = std::time::SystemTime::now();
    let time = now.duration_since(std::time::UNIX_EPOCH)?;
    let time = time.as_secs();
    macroquad::rand::srand(time);

    println!("Initializing game scene");
    let scenes: Vec<Box<dyn Scene>> = vec![Box::new(crate::game::GameScene::new())];

    Ok(scenes)
}

async fn update(scenes: &mut Vec<Box<dyn Scene>>, delta: f32)
    -> Result<bool, Box<dyn std::error::Error>>
{
    let scene = scenes.last_mut();
    if scene.is_none() {
        return Ok(false)
    }
    let scene = scene.unwrap();
    match scene.update(delta) {
        Transition::None => {},
        Transition::Remove => {
            scenes.pop();
        },
        Transition::Add(s) => {
            scenes.push(s);
        },
        Transition::Clear(s) => {
            *scenes = vec![s];
        },
        Transition::Exit => {
            return Ok(true)
        }
    };
    Ok(false)
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
    Clear(Box<dyn Scene>),
    Exit
}

pub trait Scene {
    fn update(&mut self, delta: f32) -> Transition;
    fn draw(&self);
}
