pub struct Password {
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
}

pub fn generate_salt(size: i32) -> Vec<u8> {
    let random_bytes: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();
    random_bytes
}

/// Random salt is generated everytime this function is called.
/// Hash is always different in that case.
/// Basic usage:
///
/// ```
/// use misato_security::password::*;
///
/// let encrypted_password = hash_password(b"anypassword");
/// let same_password = hash_password(b"anypassword");
/// assert_eq!(same_password.salt != encrypted_password.salt, true);
/// assert_eq!(same_password.hash != encrypted_password.hash, true);
/// ```
pub fn hash_password(password: &[u8]) -> Password {
    let salt = generate_salt(256);
    let hash = argon2::hash_raw(password, &salt, &argon2::Config::default()).unwrap();

    Password { salt, hash }
}

/// You have to provide the salt.
/// If the salt and the password are the same, the hash will be the same.
/// Basic usage:
///
/// ```
/// use misato_security::password::*;
///
/// let salt = generate_salt(256); // 256 bytes salt
/// let encrypted_password = hash_password_salt(&salt, b"anypassword");
/// let same_password = hash_password_salt(&salt, b"anypassword");
/// let another_password = hash_password_salt(&salt, b"anotherpassword");
///
/// assert_eq!(encrypted_password.salt == same_password.salt, true);
/// assert_eq!(encrypted_password.salt == another_password.salt, true);
///
/// assert_eq!(encrypted_password.hash == same_password.hash, true);
/// assert_eq!(encrypted_password.hash == another_password.hash, false);
/// ```
pub fn hash_password_salt(salt: &[u8], password: &[u8]) -> Password {
    let hash = argon2::hash_raw(password, &salt, &argon2::Config::default()).unwrap();

    println!("function salt = {:?}", salt);

    Password {
        salt: salt.iter().cloned().collect(),
        hash,
    }
}

/// Check if a plain text password is equal to a hash password
/// Basic usage:
///
/// ```
/// use misato_security::password::*;
///
/// let encrypted_password = hash_password(b"anypassword");
/// assert_eq!(is_correct_password(b"anypassword", &encrypted_password), true);
/// assert_eq!(is_correct_password(b"anotherpassword", &encrypted_password), false);
/// ```
pub fn is_correct_password(password: &[u8], target: &Password) -> bool {
    match argon2::verify_raw(
        password,
        &target.salt,
        &target.hash,
        &argon2::Config::default(),
    ) {
        Ok(result) => return result,
        Err(_) => false,
    }
}
