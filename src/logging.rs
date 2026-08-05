use crate::Result;
use chrono::Local;
use clap::Parser;
use iocore::Path;
use is_terminal::IsTerminal;
use log::LevelFilter;
use std::{fs::File, io::Write};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter as TracingLevelFilter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
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
        .chain(fern::log_file("couleur.log")?)
        // .chain(fern::Output::call(|record| {
        //     eprintln!("{}", record.args());
        // }))
        .apply()?;
    Ok(())
}

pub fn setup_tracing() -> Result<()> {
    // let file_appender =
    //     tracing_appender::rolling::hourly(env!("CARGO_MANIFEST_DIR"), "couleur-tracing.log");

    // let filter = EnvFilter::builder()
    //     .with_default_directive(TracingLevelFilter::DEBUG.into())
    //     .from_env_lossy();

    let log_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("couleur.log");
    let mut file_appender = File::options().append(true).open(log_path)?;

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),

        )
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    Ok(())
}
