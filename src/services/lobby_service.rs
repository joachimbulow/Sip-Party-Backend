use redis::Commands;

use crate::{models::vote_game::{Game, Player}, Context, Error};

pub fn is_valid_lobby_code(mut ctx: Context, code: String) -> Result<String, Error> {
    let code = ctx.state.redis_client.get::<&str, String>(&code)?;
    Ok(code)
}

pub fn create_game(mut ctx: Context, code: String, host: String) -> Result<String, Error> {
    let game = Game::new(code.to_string(), Player::new(host));
    let serialized_game = serde_json::to_string(&game).unwrap();
    let _ : () = ctx.state.redis_client.set(code.to_string(), serialized_game.to_string())?;
    Ok(serialized_game)
}

pub fn join_lobby(mut ctx: Context, code: String, joined_player: String) -> Result<String, Error> {
    let serialized_game = ctx.state.redis_client.get::<&str, String>(&code)?;
    let game: Game = serde_json::from_str(&serialized_game)?;
    game.add_player(Player::new(joined_player));
    serialized_game
    let _ : () = ctx.state.redis_client.set(code.to_string(), serialized_game.to_string())?;
    Ok(serialized_game)
}