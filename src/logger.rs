use anyhow::Result;
use chrono::Local;
use fern::Dispatch;
use log::{info, LevelFilter};
use std::{fs::File, path::Path};

pub fn init(log_path: &Path) -> Result<()> {
    // Создаём лог-файл, добавляем консоль-аутпут.
    let file = File::create(log_path)?;
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(file)
        .apply()?;
    Ok(())
}

pub fn banner() {
    info!("=== File Backup Logger started ===");
}
