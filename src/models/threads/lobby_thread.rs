
impl GameThread {
    fn new(code: String, gametype: GameType) -> Self {
        Self {
            players: Vec::new(),
            code,
            gametype,
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

impl Runnable for GameThread {
    fn run(&self) {
        // What
    }
}