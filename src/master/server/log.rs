use log::SetLoggerError;
use log::{Level, Metadata, Record};

static LOGGER: super::log::Logger = super::log::Logger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Debug))
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.metadata().level() == Level::Debug
                || record.metadata().level() == Level::Warn
                || record.metadata().level() == Level::Error
            {
                println!(
                    "[{}]({}:{}): {}",
                    record.level(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    record.args(),
                );
            } else {
                println!("[{}]: {}", record.level(), record.args(),);
            }
        }
    }

    fn flush(&self) {}
}
