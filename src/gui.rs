use crate::{backup::Backup, config::AppConfig, utils::timestamp};
use anyhow::Result;
use eframe::egui;
use log::info;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct BackupApp {
    cfg: AppConfig,
    status: String,
}

impl BackupApp {
    pub fn new(cfg: AppConfig) -> Self {
        Self {
            cfg,
            status: String::new(),
        }
    }

    fn choose_dir(current: &mut String, caption: &str) {
        if let Some(path) = FileDialog::new()
            .set_title(caption)
            .set_directory(&*current)
            .pick_folder()
        {
            *current = path.display().to_string();
        }
    }

    fn run_backup(&mut self) -> Result<()> {
        let backup = Backup {
            source: PathBuf::from(&self.cfg.source_dir),
            destination_root: PathBuf::from(&self.cfg.dest_dir),
            zip: self.cfg.default_zip,
            version: self.cfg.version_tag.clone(),
        };
        backup.run()?;
        self.status = format!("Backup completed at {}", timestamp());
        self.cfg.save()?; // сохраняем выбранные пути
        Ok(())
    }
}

impl eframe::App for BackupApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Backup Logger");

            ui.horizontal(|ui| {
                ui.label("Source:");
                ui.text_edit_singleline(&mut self.cfg.source_dir);
                if ui.button("📂").clicked() {
                    Self::choose_dir(&mut self.cfg.source_dir, "Choose Source Directory");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Destination:");
                ui.text_edit_singleline(&mut self.cfg.dest_dir);
                if ui.button("📂").clicked() {
                    Self::choose_dir(&mut self.cfg.dest_dir, "Choose Destination Directory");
                }
            });

            ui.checkbox(&mut self.cfg.default_zip, "ZIP compression");

            ui.horizontal(|ui| {
                ui.label("Version tag:");
                ui.text_edit_singleline(&mut self.cfg.version_tag);
            });

            if ui.button("Run Backup").clicked() {
                match self.run_backup() {
                    Ok(_) => info!("{}", self.status),
                    Err(e) => {
                        self.status = format!("❌ {}", e);
                        info!("{}", self.status);
                    }
                }
            }

            ui.separator();
            ui.label(&self.status);
        });
    }
}
