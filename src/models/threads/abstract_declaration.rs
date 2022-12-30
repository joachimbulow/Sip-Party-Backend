
pub enum GameType {
    Lobby,
    VoteGame
}

pub struct GameThread {
    players: Vec<Player>,
    code: String, 
    gametype: GameType
}

trait Runnable {
    fn run(&self);
}

trait Receiving {
    fn onRecieve(data: &InboundData);
}



