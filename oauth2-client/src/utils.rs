use rand::{distributions::Alphanumeric, thread_rng, Rng as _};

pub fn gen_state() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>()
}
