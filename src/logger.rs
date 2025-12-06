use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

pub fn init(verbose: bool) {
    let level = if verbose {
        LevelFilter::max()
    } else {
        LevelFilter::Warn
    };
    Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            let warn_style = buf.default_level_style(record.level());
            let level = record.level().to_string();

            writeln!(buf, "{warn_style}{level}:{warn_style:#}{}", record.args())
        })
        .init();
}
