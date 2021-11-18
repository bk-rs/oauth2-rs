use sha2::{Digest as _, Sha256};

use crate::types::{CodeChallenge, CodeChallengeMethod, CodeVerifier};

pub fn gen_code_challenge(
    code_verifier: CodeVerifier,
    code_challenge_method: impl Into<Option<CodeChallengeMethod>>,
) -> (CodeChallenge, CodeChallengeMethod) {
    let code_challenge_method: CodeChallengeMethod =
        code_challenge_method.into().unwrap_or_default();

    let code_challenge = match code_challenge_method {
        CodeChallengeMethod::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(code_verifier.as_bytes());
            let result = hasher.finalize();
            base64::encode(&result[..])
        }
        CodeChallengeMethod::Plain => code_verifier,
    };

    (code_challenge, code_challenge_method)
}
