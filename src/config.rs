use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct MfetchConfig {
    pub theme: String,
    pub show_spd_source: bool,
}

impl Default for MfetchConfig {
    fn default() -> Self {
        Self {
            theme: "catppuccin".to_string(),
            show_spd_source: true,
        }
    }
}

pub fn load() -> MfetchConfig {
    let home_dir = if let Ok(user) = env::var("SUDO_USER") {
        format!("/home/{}", user)
    } else {
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    };

    let config_path = PathBuf::from(home_dir)
        .join(".config")
        .join("mfetch")
        .join("config.toml");

    if let Ok(content) = fs::read_to_string(&config_path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        let default_cfg = MfetchConfig::default();
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&config_path, toml::to_string(&default_cfg).unwrap());
        default_cfg
    }
}
