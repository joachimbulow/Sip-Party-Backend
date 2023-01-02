use redis::Commands;

use crate::{models::vote_game::{Game, Player}, Context};

pub fn is_valid_lobby_code(mut ctx: Context, code: String) -> bool {
    let result = ctx.state.redis_client.get::<&str, String>(&code);
    match result {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn create_game(mut ctx: Context, code: String, host: String) {
    let game = Game::new(code, Player::new(host));
}