// TODO: remove this
#![allow(unused_variables)]
#![allow(dead_code)]

pub struct Game {
    rounds: Vec<Round>,
    players: Vec<Player>,
    host: Player
}

impl Game {
    fn new(host: Player) -> Game {
        Game { rounds: Vec::new(), players: Vec::new(), host: host }
    }
}

pub struct Round {
    votes: Vec<Vote>,
    question: Question,
}

impl Round {
    fn new(question: Question) -> Round {
        Round { votes: Vec::new(), question: question }
    }

    fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }
}

pub struct Player {
    name: String,
    avatar_url: String,
}

// TODO: Implement avatars a cloud bucket
impl Player {
    fn new(name: String) -> Player {
        Player { name: name, avatar_url: String::from("none") }
    }
}

pub struct Vote {
    sender: Player,
    receiver: Player
}

impl Vote {
    fn new(sender: Player, receiver: Player) -> Vote {
        Vote { sender: sender, receiver: receiver }
    }
}

pub struct Question {
    text: String,
}

impl Question {
    fn new(text: String) -> Question {
        Question { text: text }
    }
}
