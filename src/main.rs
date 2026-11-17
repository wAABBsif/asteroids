#[macroquad::main("Asteroids")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    asteroids::scenes::run().await?;
    Ok(())
}
