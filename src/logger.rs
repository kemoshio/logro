use anyhow::{anyhow, Result};

#[cfg(target_os = "macos")]
enum Color {
    White,
    Red,
    Green,
    Yellow,
    Cyan,
}

#[cfg(target_os = "macos")]
impl Color {
    fn v(&self) -> u8 {
        match self {
            Color::White => 0,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Cyan => 36,
        }
    }
}

// 0 Off、 1 Trace、2 Debug、3 Info、4 Warn、5 Error
pub fn setup(level: i32) -> Result<()> {
    return match level {
        1 => {
            setup_logger(log::LevelFilter::Trace)
        }
        2 => {
            setup_logger(log::LevelFilter::Debug)
        }
        3 => {
            setup_logger(log::LevelFilter::Info)
        }
        4 => {
            setup_logger(log::LevelFilter::Warn)
        }
        5 => {
            setup_logger(log::LevelFilter::Error)
        }
        _ => {
            setup_logger(log::LevelFilter::Off)
        }
    };
}

pub fn setup_logger(level: log::LevelFilter) -> Result<()> {
    let mut dispatch = fern::Dispatch::new();

    #[cfg(any(target_os = "ios", target_os = "android"))] {
        dispatch = dispatch
            .format(|out, message, record| {
                let vec: Vec<&str> = record.target().split("::").collect();
                let target_last = vec[vec.len() - 1];
                let target_line = format!("{}:{}", target_last, record.line().unwrap());
                out.finish(format_args!(
                    "[R] {date} {level:>level_size$} {target:<target_size$} --- {message}",
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    level = record.level(),
                    level_size = 5,
                    target = target_line,
                    target_size = 14,
                    message = message,
                ))
            })
            .level(level);

        let console_output = fern::Output::writer(
            Box::new(crate::mobile::logger::ConsoleWriter::default()),
            "\n",
        );

        dispatch = dispatch.chain(console_output);
    }

    #[cfg(target_os = "macos")] {
        dispatch = dispatch
            .format(|out, message, record| {
                let vec: Vec<&str> = record.target().split("::").collect();
                let target_last = vec[vec.len() - 1];
                let target_line = format!("{}:{}", target_last, record.line().unwrap());
                let lev_color = match record.level() {
                    log::Level::Debug => Color::White,
                    log::Level::Trace => Color::White,
                    log::Level::Info => Color::Green,
                    log::Level::Warn => Color::Yellow,
                    log::Level::Error => Color::Red
                };
                out.finish(format_args!(
                    "{date} \x1B[{level_color}m{level:>level_size$}\x1B[0m \x1B[{target_color}m{target:<target_size$}\x1B[0m --- {message}",
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    level_color = lev_color.v(),
                    level = record.level(),
                    level_size = 5,
                    target_color = Color::Cyan.v(),
                    target = target_line,
                    target_size = 14,
                    message = message,
                ))
            })
            .level(level);

        dispatch = dispatch.chain(std::io::stdout());
    }

    if let Err(e) = dispatch.apply() {
        return Err(anyhow!("Apply logger config failed: {}", e));
    }

    Ok(())
}