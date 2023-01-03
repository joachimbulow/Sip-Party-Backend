use rand::{thread_rng, Rng};

pub fn generate_code() -> String {
    let mut result = String::new();
    let mut rng = thread_rng();
    for _ in 0..6 {
        let ch = char::from_u32(thread_rng().gen_range(48..58)).unwrap();
        result.push(ch);
    }
    result
}
