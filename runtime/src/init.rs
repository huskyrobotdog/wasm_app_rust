use log::LevelFilter;
#[cfg(feature = "dev")]
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

pub fn log() {
    #[cfg(feature = "dev")]
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new().set_time_format_rfc3339().build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
