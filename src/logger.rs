use log::{Level, LevelFilter};

use env_logger::{
    fmt::{Color, Style, StyledValue},
    Builder,
};

fn colored_level(style: &mut Style, level: Level) -> StyledValue<&'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

pub fn init() {
    let mut builder = Builder::new();

    builder.format(|f, record| {
        use std::io::Write;

        let mut style = f.style();
        let level = colored_level(&mut style, record.level());
        writeln!(f, "{}> {}", level, record.args())
    });

    builder.filter_level(LevelFilter::Info).init();
}
