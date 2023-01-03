use std::result;

use crate::utils;
use crate::{
    models::vote_game::{Game, Player},
    router::Router,
    services, Context,
};
use redis::Commands;

pub fn init_lobby_routes(router: &mut Router) {
    router.get(
        "/is_valid_lobby_code/:lobby_code",
        Box::new(is_valid_lobby_code),
    );
    router.get("/join_lobby/:lobby_code/:name", Box::new(join_lobby));
    router.post("/host_lobby/:name", Box::new(host_lobby));
}

pub async fn is_valid_lobby_code(ctx: Context) -> String {
    let code = match ctx.params.find("lobby_code") {
        Some(v) => v.to_string(),
        None => "empty".to_string(),
    };

    match services::lobby_service::is_valid_lobby_code(ctx, code.to_string()) {
        Ok(_) => String::from("true"),
        Err(_) => String::from("false"),
    }
}

//TODO: Add a name
pub async fn host_lobby(mut ctx: Context) -> String {
    let code: String = utils::generate_code();

    // Name
    let name = match ctx.params.find("name") {
        Some(v) => v.to_string(),
        None => "empty".to_string(),
    };

    // Creating the game
    let created_game = match services::lobby_service::create_game(ctx, code.to_string(), name.to_string()) {
        Ok(game) => game,
        Err(_) => String::from("An error occured"),
    };

    created_game
}


pub async fn join_lobby(mut ctx: Context) -> String {
    let code = match ctx.params.find("lobby_code") {
        Some(v) => v,
        None => "empty",
    };

    let is_valid = match services::lobby_service::is_valid_lobby_code(ctx, code.to_string()) {
        Ok(_) => true,
        Err(_) => false,
    };

    // TODO: Return the serialized game so that the frontend can display the game
    let result = ctx.state.redis_client.get::<&str, String>(&code);
    match result {
        Ok(_) => String::from("true"),
        Err(_) => String::from("false"),
    }
}
