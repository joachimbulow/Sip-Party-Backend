
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    score: u32
}

impl Player {
    fn new(name: String) -> Self {
        Self {name, score: 0 }
    }

    fn add_score(&mut self, points: u32) {
        self.score += points;
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }
}