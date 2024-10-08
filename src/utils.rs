use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
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
