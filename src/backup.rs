use crate::utils::{backup_dir_name, timestamp};
use anyhow::{anyhow, Context, Result};
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use log::info;
use std::{
    fs::{self, File},
    io::{Write, Read},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

pub struct Backup {
    pub source: PathBuf,
    pub destination_root: PathBuf,
    pub zip: bool,
    pub version: String,
}

impl Backup {
    pub fn run(&self) -> Result<()> {
        // Проверки
        if !self.source.exists() {
            return Err(anyhow!("Source directory does not exist"));
        }
        let backup_dir = self.destination_root.join(backup_dir_name(&self.version));
        fs::create_dir_all(&backup_dir)?;

        let start = std::time::Instant::now();
        let mut file_count = 0_u64;

        if self.zip {
            let zip_path = backup_dir.join(format!("{}.zip", timestamp()));
            self.create_zip(&zip_path, &mut file_count)?;
            info!(
                "ZIP backup completed: {} files, duration: {:.2?}",
                file_count,
                start.elapsed()
            );
        } else {
            self.copy_plain(&backup_dir, &mut file_count)?;
            info!(
                "Plain copy completed: {} files, duration: {:.2?}",
                file_count,
                start.elapsed()
            );
        }

        Ok(())
    }

    fn copy_plain(&self, backup_dir: &Path, file_count: &mut u64) -> Result<()> {
        let mut opts = CopyOptions::new();
        opts.copy_inside = true;
        copy_dir(&self.source, backup_dir, &opts).context("Copy failed")?;
        // Посчитаем файлы
        for entry in WalkDir::new(backup_dir).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                *file_count += 1;
            }
        }
        Ok(())
    }

    fn create_zip(&self, zip_path: &Path, file_count: &mut u64) -> Result<()> {
        let file = File::create(zip_path)?;
        let mut zip = ZipWriter::new(file);
        let options: FileOptions<'static, ()> = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        let base = self.source.parent().unwrap_or(&self.source);

        for entry in WalkDir::new(&self.source).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            let name = path.strip_prefix(base)?.to_str().unwrap();

            if path.is_dir() {
                zip.add_directory(name, options)?;
            } else {
                zip.start_file(name, options)?;
                let mut f = File::open(path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                *file_count += 1;
            }
        }

        zip.finish()?;
        Ok(())
    }
}
