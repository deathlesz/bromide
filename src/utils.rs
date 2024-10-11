use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use base64::{prelude::BASE64_URL_SAFE, Engine as _};
use sha1_smol::Sha1;

pub fn password_hash(password: impl AsRef<[u8]>) -> String {
    // we're storing sha-1 has of the password
    // which is then hashed by argon2id
    // as sha-1 is not cryptographically secure
    let mut sha1 = Sha1::from(password);
    sha1.update(b"mI29fmAnxgTs");

    let salt = SaltString::generate(&mut OsRng);

    // 19 MiB of memory, 2 iterations, 1 degree of parallelism
    let argon2 = Argon2::default();

    argon2
        .hash_password(&sha1.digest().bytes(), &salt)
        .expect("argon2id shouldn't fail")
        .to_string()
}

pub fn check_password(password: &str, gjp2: &str) -> bool {
    let argon2 = Argon2::default();

    argon2
        .verify_password(
            &hex::decode(gjp2).expect("hex decoding should never fail"),
            &PasswordHash::new(password).expect("converting to password hash shouldn't fail"),
        )
        .is_ok()
}

// this is just... awful
// but it's convinient so idc
pub trait ToStringAndDisplay: ToString + std::fmt::Display {}
impl<T: ToString + std::fmt::Display> ToStringAndDisplay for T {}

pub fn generate_chk(
    params: &[&dyn ToStringAndDisplay],
    salt: impl AsRef<[u8]>,
    key: impl AsRef<[u8]>,
) -> String {
    let concatenated = params
        .iter()
        .fold(String::with_capacity(20), |acc, p| format!("{acc}{p}"));

    let mut sha1 = Sha1::from(concatenated);
    sha1.update(salt.as_ref());

    let xored = xor_cipher(sha1.hexdigest(), key);

    BASE64_URL_SAFE.encode(xored)
}

pub fn xor_cipher(input: impl AsRef<[u8]>, key: impl AsRef<[u8]>) -> Vec<u8> {
    input
        .as_ref()
        .iter()
        .zip(key.as_ref().iter().cycle())
        .map(|(byte, key)| byte ^ key)
        .collect()
}
