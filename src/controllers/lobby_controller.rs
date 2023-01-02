use std::result;

use crate::{router::Router, Context, services};
use redis::Commands;
use crate::utils;

pub fn init_lobby_routes(router: &mut Router) {
    router.get("/is_valid_lobby_code/:lobby_code", Box::new(is_valid_lobby_code));
    router.get("/join_lobby/:lobby_code/:name", Box::new(join_lobby));
    router.post("/host_lobby/:name", Box::new(host_lobby));

}

pub async fn is_valid_lobby_code(mut ctx: Context) -> String {
    let code = match ctx.params.find("lobby_code") {
        Some(v) => v.to_string(),
        None => "empty".to_string(),
    };

    match services::lobby_service::is_valid_lobby_code(ctx, String::from(code)) {
        true => String::from("true"),
        false => String::from("false")
    }
}

//TODO: Add a name
pub async fn host_lobby(mut ctx: Context) -> String {
    let code: String = utils::generate_code();
    let result: Result<String, redis::RedisError> = ctx.state.redis_client.set(code.to_string(), String::from("this is a game that needs to be serialized"));
    match result {
        Ok(_) => String::from(code),
        Err(_) => String::from("throw some error soon..."),
    }
}

pub async fn join_lobby(mut ctx: Context) -> String {
    let code = match ctx.params.find("lobby_code") {
        Some(v) => v,
        None => "empty",
    };

    // TODO: Return the serialized game so that the frontend can display the game
    let result = ctx.state.redis_client.get::<&str, String>(&code);
    match result {
        Ok(_) => String::from("true"),
        Err(_) => String::from("false")
    }
}