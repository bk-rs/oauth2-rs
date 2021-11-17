use oauth2_core::types::{Nonce, State};
use rand::{distributions::Alphanumeric, thread_rng, Rng as _};

pub fn gen_state(length: impl Into<Option<usize>>) -> State {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.into().unwrap_or(10))
        .map(char::from)
        .collect::<String>()
}

pub fn gen_nonce(length: impl Into<Option<usize>>) -> Nonce {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.into().unwrap_or(22))
        .map(char::from)
        .collect::<String>()
}
