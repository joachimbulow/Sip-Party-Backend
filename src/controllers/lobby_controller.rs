use crate::{router::Router, Context};
use redis::Commands;
use crate::utils;

pub fn init_lobby_routes(router: &mut Router) {
    router.get("/is_valid_lobby_code/:lobby_code", Box::new(is_valid_lobby_code));
    router.get("/join_lobby/:lobby_code", Box::new(join_lobby));
    router.post("/host_lobby", Box::new(host_lobby));

}

pub async fn is_valid_lobby_code(mut ctx: Context) -> String {
    let code = match ctx.params.find("lobby_code") {
        Some(v) => v,
        None => "empty",
    };
    let result = ctx.state.redis_client.get::<&str, String>(code);
    match result {
        Ok(_) => String::from("true"),
        Err(_) => String::from("false")
    }
}

pub async fn host_lobby(mut ctx: Context) -> String {
    let code: String = utils::generate_code();
    let result = ctx.state.redis_client.set(&code, String::from("this is a game that needs to be serialized"));
    match result {
        Ok(code) => code,
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