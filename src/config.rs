use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub source_dir: String,
    pub dest_dir: String,
    pub default_zip: bool,
    pub version_tag: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            source_dir: String::new(),
            dest_dir: String::new(),
            default_zip: true,
            version_tag: "1.0.0".into(),
        }
    }
}

impl AppConfig {
    fn config_path() -> Result<PathBuf> {
        let proj_dirs =
            ProjectDirs::from("com", "example", "FileBackupLogger").context("No project dirs")?;
        let path = proj_dirs.config_dir().join("config.json");
        fs::create_dir_all(path.parent().unwrap())?;
        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
