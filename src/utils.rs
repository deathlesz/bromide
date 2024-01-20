use base64::prelude::{Engine as _, BASE64_URL_SAFE};
use sha1_smol::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub(crate) fn timestamp_to_relative(timestamp: u64) -> String {
    let since_unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
    let mut diff = since_unix_epoch.as_secs() - timestamp;

    if diff <= 44 {
        return "a few seconds".to_string();
    } else if diff <= 89 {
        return "a minute".to_string();
    }

    diff /= 60;
    if diff <= 44 {
        return format!("{} minutes", diff);
    } else if diff <= 89 {
        return "an hour".to_string();
    }

    diff /= 60;
    if diff <= 21 {
        return format!("{} hours", diff);
    } else if diff <= 35 {
        return "a day".to_string();
    }

    diff /= 24;
    if diff <= 25 {
        return format!("{} days", diff);
    } else if diff <= 45 {
        return "a month".to_string();
    }

    diff /= 30;
    if diff <= 10 {
        return format!("{} months", diff);
    } else if diff <= 17 {
        return "a year".to_string();
    }

    let diff = (diff as f64 / 12.0).round() as i64;
    format!("{:.0} years", diff)
}
pub(super) async fn shutdown_signal() {
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
