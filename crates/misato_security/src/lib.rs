use rand::{distributions::Alphanumeric, Rng};

pub mod password;

pub fn generate_token(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
