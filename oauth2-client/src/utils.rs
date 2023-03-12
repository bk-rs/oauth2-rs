use core::cmp::{max, min};

use oauth2_core::types::{
    code_verifier::{CODE_VERIFIER_CHARSET, CODE_VERIFIER_LEN_MAX, CODE_VERIFIER_LEN_MIN},
    CodeVerifier, Nonce, State,
};
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

pub fn gen_code_verifier(length: impl Into<Option<usize>>) -> CodeVerifier {
    let length = length.into().unwrap_or(64);
    let length = min(CODE_VERIFIER_LEN_MAX, length);
    let length = max(CODE_VERIFIER_LEN_MIN, length);

    let mut rng = thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CODE_VERIFIER_CHARSET.len());
            CODE_VERIFIER_CHARSET[idx] as char
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_code_verifier() {
        assert_eq!(gen_code_verifier(64).len(), 64);
    }
}
