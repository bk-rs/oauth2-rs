use sha2::{Digest as _, Sha256};

use crate::types::{
    code_verifier::{CODE_VERIFIER_LEN_MAX, CODE_VERIFIER_LEN_MIN},
    CodeChallenge, CodeChallengeMethod, CodeVerifier,
};

// Ref https://github.com/ramosbugs/oauth2-rs/blob/4.1.0/src/types.rs#L498
pub fn gen_code_challenge(
    code_verifier: CodeVerifier,
    code_challenge_method: impl Into<Option<CodeChallengeMethod>>,
) -> (CodeChallenge, CodeChallengeMethod) {
    assert!(
        code_verifier.len() >= CODE_VERIFIER_LEN_MIN
            && code_verifier.len() <= CODE_VERIFIER_LEN_MAX
    );

    let code_challenge_method: CodeChallengeMethod =
        code_challenge_method.into().unwrap_or_default();

    let code_challenge = match code_challenge_method {
        CodeChallengeMethod::Sha256 => {
            let digest = Sha256::digest(code_verifier.as_bytes());
            base64::encode_config(digest, base64::URL_SAFE_NO_PAD)
        }
        CodeChallengeMethod::Plain => code_verifier,
    };

    (code_challenge, code_challenge_method)
}
