const COMBO_TIME: f32 = 1.0;

pub struct ScoreManager {
    pub score: u64,
    pub combo: u32,
    pub combo_timer: f32
}

impl ScoreManager {
    pub fn new() -> ScoreManager {
        ScoreManager {
            score: 0,
            combo: 0,
            combo_timer: 0.0
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.combo_timer -= delta;
        if self.combo_timer < 0.0 {
            self.combo = 0;
        }
        println!("Score: {}, Combo: {}", self.score, self.combo);
    }

    pub fn increase_combo(&mut self) {
        self.combo += 1;
        self.combo_timer = COMBO_TIME;
    }
}
