use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MfetchConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub show_system: bool,
    #[serde(default = "default_true")]
    pub show_cpu: bool,
    #[serde(default = "default_true")]
    pub show_spd_source: bool,
    #[serde(default = "default_true")]
    pub show_swap: bool,
    #[serde(default = "default_true")]
    pub show_hugepages: bool,
    #[serde(default = "default_true")]
    pub show_pressure: bool,
    #[serde(default = "default_true")]
    pub show_modules: bool,
    #[serde(default = "default_true")]
    pub show_gpu: bool,
}

fn default_theme() -> String {
    "catppuccin".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for MfetchConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            show_system: true,
            show_cpu: true,
            show_spd_source: true,
            show_swap: true,
            show_hugepages: true,
            show_pressure: true,
            show_modules: true,
            show_gpu: true,
        }
    }
}

fn home_dir() -> PathBuf {
    if let Ok(user) = env::var("SUDO_USER") {
        if let Ok(output) = Command::new("getent").args(["passwd", &user]).output() {
            let line = String::from_utf8_lossy(&output.stdout);
            if let Some(home) = line.split(':').nth(5) {
                let home = home.trim();
                if !home.is_empty() {
                    return PathBuf::from(home);
                }
            }
        }
        PathBuf::from(format!("/home/{}", user))
    } else {
        PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()))
    }
}

pub fn load() -> MfetchConfig {
    let config_path = home_dir()
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

#[derive(Debug)]
pub struct Args {
    pub debug: bool,
}

pub fn parse_args() -> Args {
    let mut debug = false;
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--debug" => debug = true,
            "--version" | "-V" => {
                println!("mfetch v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => {}
        }
    }
    Args { debug }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let cfg = MfetchConfig::default();
        assert_eq!(cfg.theme, "catppuccin");
        assert!(cfg.show_system);
        assert!(cfg.show_cpu);
        assert!(cfg.show_spd_source);
        assert!(cfg.show_swap);
        assert!(cfg.show_hugepages);
        assert!(cfg.show_modules);
    }

    #[test]
    fn test_parse_toml() {
        let toml = r#"
theme = "nord"
show_spd_source = false
show_swap = false
show_hugepages = false
"#;
        let cfg: MfetchConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.theme, "nord");
        assert!(!cfg.show_spd_source);
        assert!(!cfg.show_swap);
        assert!(!cfg.show_hugepages);
        assert!(cfg.show_modules);
        assert!(cfg.show_system);
    }

    #[test]
    fn test_parse_empty() {
        let cfg: MfetchConfig = toml::from_str("").unwrap();
        assert_eq!(cfg.theme, "catppuccin");
        assert!(cfg.show_spd_source);
    }
}
