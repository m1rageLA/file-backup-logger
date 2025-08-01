mod backup;
mod config;
mod gui;
mod logger;
mod utils;

use crate::{backup::Backup, config::AppConfig};
use anyhow::Result;
use clap::{Parser, Subcommand};
use logger::banner;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "File Backup Logger on Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run backup from source to destination
    Run {
        /// Source directory
        #[arg(short, long)]
        src: PathBuf,
        /// Destination root directory
        #[arg(short, long)]
        dst: PathBuf,
        /// Zip compression
        #[arg(short, long, default_value_t = true)]
        zip: bool,
        /// Version tag
        #[arg(short, long, default_value_t = String::from("1.0.0"))]
        ver: String,
    },
    /// Launch GUI
    Gui,
}

fn main() -> Result<()> {
    // Настраиваем лог-файл в каталоге программы
    let log_path = std::env::current_dir()?.join("backup.log");
    logger::init(&log_path)?;
    banner();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { src, dst, zip, ver }) => {
            let backup = Backup {
                source: src,
                destination_root: dst,
                zip,
                version: ver,
            };
            backup.run()?;
        }
        _ => {
            // GUI by default
            let cfg = AppConfig::load()?;
            let options = eframe::NativeOptions::default();
            eframe::run_native(
                "File Backup Logger",
                options,
                Box::new(|_cc| Ok(Box::new(gui::BackupApp::new(cfg)))),
            );
        }
    }

    Ok(())
}
