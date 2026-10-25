use asteroids::game;

#[macroquad::main("Asteroids")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    return game::run().await;
}
