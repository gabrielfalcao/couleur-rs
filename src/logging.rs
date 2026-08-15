use std::fs::File;

// use std::io::Write;
#[cfg(any(feature = "logging", feature = "tracing"))] use chrono::Local;
use iocore::{Path, env};
#[cfg(any(feature = "logging", feature = "tracing"))] use log::LevelFilter;
#[cfg(any(feature = "logging", feature = "tracing"))]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{Error, Result};

#[cfg(any(feature = "logging", feature = "tracing"))]
pub fn setup_logging() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] pid:{} {}: {}",
                Local::now().to_rfc3339_opts(chrono::format::SecondsFormat::Secs, true),
                record.level(),
                std::process::id(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(fern::log_file(get_log_path()?)?)
        // .chain(fern::Output::call(|record| {
        //     eprintln!("{}", record.args());
        // }))
        .apply()?;
    Ok(())
}
#[cfg(not(any(feature = "logging", feature = "tracing")))]
pub fn setup_logging() -> Result<()> {
    Ok(())
}

pub fn local_data_dir() -> Path {
    dirs::data_local_dir()
        .map(|data_local_dir| Path::from(data_local_dir))
        .unwrap_or_else(|| Path::cwd())
        .join("couleur_rs")
}

#[cfg(any(feature = "logging", feature = "tracing"))]
#[cfg(not(feature = "trace-test-debug"))]
pub fn get_log_path() -> Result<Path> {
    let log_path =
        if let Ok(log_path) = env::var("COULEUR_LOG_PATH").map(|log_path| Path::new(log_path)) {
            log_path
        } else {
            let log_dir = env::var("COULEUR_LOG_DIR")
                .ok()
                .map(|couleur_log_dir| Path::new(couleur_log_dir))
                .unwrap_or_else(|| local_data_dir());
            let log_filename =
                env::var("COULEUR_LOG_FILENAME").ok().unwrap_or_else(|| "couleur.log".to_string());

            log_dir.join(log_filename)
        };
    if !log_path.exists() {
        log_path.write(b"").map_err(|error| {
            Error::InitializationError(format!(
                "couleur log path is not writable {log_path}: {error}",
                log_path = log_path.to_string()
            ))
        })?;
        log_path.delete()?;
    }
    Ok(log_path)
}
#[cfg(any(feature = "logging", feature = "tracing"))]
#[cfg(feature = "trace-test-debug")]
pub fn get_log_path() -> Result<Path> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR")).join("couleur.log"))
}

#[cfg(not(any(feature = "logging", feature = "tracing")))]
pub fn get_log_path() -> Result<Path> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR")).join("couleur.log"))
}

#[cfg(feature = "tracing")]
pub fn setup_tracing() -> Result<()> {
    let log_path = get_log_path()?;
    let file_appender = File::options().append(true).open(log_path)?;

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    Ok(())
}
#[cfg(not(feature = "tracing"))]
pub fn setup_tracing() -> Result<()> {
    Ok(())
}
