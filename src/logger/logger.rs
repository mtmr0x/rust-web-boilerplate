extern crate log;
extern crate chrono;
extern crate fern;

use std::io;
use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .error(Color::Red)
        .warn(Color::Yellow)
        .debug(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
        ))
    })
    .level(log::LevelFilter::Debug)
    .chain(io::stdout())
    .chain(fern::log_file("output.log")?)
    .apply()?;
    Ok(())
}

