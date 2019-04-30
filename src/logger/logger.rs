extern crate log;
extern crate chrono;
extern crate fern;

use std::io;
use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger(verbosity:u8) -> Result<(), fern::InitError> {
    let mut level_config = fern::Dispatch::new();

    level_config = match verbosity {
        0 => level_config.level(log::LevelFilter::Warn),
        1 => level_config.level(log::LevelFilter::Info),
        _2_or_more => level_config.level(log::LevelFilter::Debug),
    };

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .error(Color::Red)
        .warn(Color::Yellow)
        .debug(Color::Magenta);

    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
        ))
    })
    .chain(io::stdout())
    .chain(fern::log_file("output.log")?);

    level_config.chain(stdout_config).apply()?;

    Ok(())
}

