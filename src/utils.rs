use base64::prelude::{Engine as _, BASE64_URL_SAFE};
use sha1_smol::Sha1;

pub(crate) fn password_hash(password: &str) -> String {
    let mut sha1 = Sha1::from(password);
    sha1.update(b"mI29fmAnxgTs");

    sha1.hexdigest()
}

pub(crate) fn generate_chk<T, K>(params: &[T], salt: &str, key: K) -> Option<String>
where
    T: ToString,
    K: AsRef<[u8]>,
{
    let concatenated = params
        .iter()
        .map(ToString::to_string)
        .reduce(|acc, e| format!("{acc}{e}"))?;

    let mut sha1 = Sha1::from(concatenated);
    sha1.update(salt.as_bytes());

    let xored = xor_cipher(sha1.hexdigest(), key);

    Some(BASE64_URL_SAFE.encode(xored))
}

// This is beatiful.
pub(crate) fn xor_cipher<S, K>(string: S, key: K) -> Vec<u8>
where
    S: AsRef<[u8]>,
    K: AsRef<[u8]>,
{
    string
        .as_ref()
        .iter()
        .zip(key.as_ref().iter().cycle())
        .map(|(byte, key)| byte ^ key)
        .collect()
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
