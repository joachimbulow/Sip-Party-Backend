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

    // TODO: implement start game etc.
}

pub struct Round {
    votes: Vec<Vote>,
    question: Question,
}

impl Round {
    fn new(question: Question) -> Round {
        Round { votes: Vec::new(), question: question }
    }

    fn vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }
}

pub struct Player {
    name: String,
    avatar_url: String,
}

pub struct Vote {
    sender: Player,
}

pub struct Question {
    text: String,
}
