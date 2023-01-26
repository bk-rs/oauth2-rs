pub mod config;
pub mod context;

//
pub mod helpers {
    pub fn state_session_key(provider: &str) -> String {
        format!("state_{provider}")
    }

    pub fn nonce_session_key(provider: &str) -> String {
        format!("nonce_{provider}")
    }

    pub fn code_verifier_session_key(provider: &str) -> String {
        format!("code_verifier_{provider}")
    }
}
