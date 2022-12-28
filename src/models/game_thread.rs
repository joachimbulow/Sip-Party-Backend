
pub enum GameType {
    Lobby,
    VoteGame
}

pub struct GameThread {
    players: Vec<Player>,
    code: String, 
    onReceive: fn(InboundData),


}


