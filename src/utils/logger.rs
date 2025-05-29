use log::LevelFilter;
use env_logger::Builder;

pub fn init_logger() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .init();
}
