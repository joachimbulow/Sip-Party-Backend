// TODO: remove this
#![allow(unused_variables)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    code: String,
    rounds: Vec<Round>,
    players: Vec<Player>,
    host: Player
}

impl Game {
    pub fn new(code: String, host: Player) -> Game {
        Game { code: code, rounds: Vec::new(), players: Vec::new(), host: host }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Round {
    votes: Vec<Vote>,
    question: Question,
}

impl Round {
    pub fn new(question: Question) -> Round {
        Round { votes: Vec::new(), question: question }
    }

    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    avatar_url: String,
}

// TODO: Implement avatars a cloud bucket
impl Player {
    pub fn new(name: String) -> Player {
        Player { name: name, avatar_url: String::from("none") }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    sender: Player,
    receiver: Player
}

impl Vote {
    pub fn new(sender: Player, receiver: Player) -> Vote {
        Vote { sender: sender, receiver: receiver }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    text: String,
}

impl Question {
    pub fn new(text: String) -> Question {
        Question { text: text }
    }
}
