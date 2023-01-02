fn is_valid_lobby_code(code: String) -> bool {
    let result = ctx.state.redis_client.get::<&str, String>(code);
    match result {
        Ok(_) => true,
        Err(_) => false
    }
}