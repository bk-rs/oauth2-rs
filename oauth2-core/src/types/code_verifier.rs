pub const CODE_VERIFIER_LEN_MIN: usize = 43;
pub const CODE_VERIFIER_LEN_MAX: usize = 128;

pub const CODE_VERIFIER_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                            abcdefghijklmnopqrstuvwxyz\
                                            0123456789-._~";

pub type CodeVerifier = String;
