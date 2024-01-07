#[cfg(feature = "argon2")]
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
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

pub(crate) async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
