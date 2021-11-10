use rand::{distributions::Alphanumeric, thread_rng, Rng as _};

pub fn gen_state(length: impl Into<Option<usize>>) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.into().unwrap_or_else(|| 10))
        .map(char::from)
        .collect::<String>()
}
