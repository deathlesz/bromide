#[cfg(feature = "argon2")]
use argon2::{
    Argon2,
    password_hash::{rand_core::OsRng, SaltString}, PasswordHasher,
};
use sha1_smol::Sha1;

#[cfg(feature = "argon2")]
pub(crate) fn password_hash(password: &str) -> argon2::password_hash::Result<String> {
    let mut sha1 = Sha1::from(password);
    sha1.update(b"mI29fmAnxgTs");
    let digest = sha1.digest().bytes();

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon2.hash_password(&digest, &salt)?.to_string())
}

#[cfg(not(feature = "argon2"))]
pub(crate) fn password_hash(password: &str) -> String {
    let mut sha1 = Sha1::from(password);
    sha1.update(b"mI29fmAnxgTs");

    sha1.hexdigest()
}
