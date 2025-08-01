use chrono::{DateTime, Local};

pub fn timestamp() -> String {
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}

pub fn backup_dir_name(version: &str) -> String {
    format!("backup_{}_v{}", timestamp(), version)
}
