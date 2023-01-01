use crate::{router::Router, Context};
use redis::Commands;


pub fn init_lobby_routes(router: &mut Router) {
    router.get("/is_valid_lobby_code/:lobby_code", Box::new(is_valid_lobby_code));
    router.get("/join_lobby", Box::new(join_lobby));
    router.post("/host_lobby", Box::new(host_lobby));

}

pub async fn is_valid_lobby_code(ctx: Context) -> String {
    // TODO: implement this
    let code = match ctx.params.find("lobby_code") {
        Some(v) => v,
        None => "empty",
    };
    println!("is valid");
    format!("Heyy")
}

pub async fn host_lobby(mut ctx: Context) -> String {
    // TODO: implement this
    println!("host");
    let result = ctx.state.redis_client.get::<&str, String>("hey");
    match result {
        Ok(value) => println!("value from redis is {}", value),
        Err(err) => println!("there was likely no key in db {}", err)
    }
    format!("Heyy")
}

pub async fn join_lobby(ctx: Context) -> String {
    // TODO: implement this
    println!("join");
    format!("Heyy")
}