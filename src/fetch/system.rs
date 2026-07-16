use std::fs;

pub struct SysInfo {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub uptime: String,
}

pub fn get() -> Option<SysInfo> {
    let hostname = fs::read_to_string("/proc/sys/kernel/hostname")
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let kernel = fs::read_to_string("/proc/version")
        .ok()
        .map(|s| s.split_whitespace().take(3).collect::<Vec<_>>().join(" "))
        .unwrap_or_default();

    let os = os_release().unwrap_or_else(|| "linux".to_string());

    let uptime = uptime();

    if hostname.is_empty() && kernel.is_empty() {
        return None;
    }

    Some(SysInfo {
        hostname,
        os,
        kernel,
        uptime,
    })
}

fn os_release() -> Option<String> {
    let raw = fs::read_to_string("/etc/os-release").ok()?;
    let mut name = None;
    let mut version = None;

    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("PRETTY_NAME=") {
            name = Some(unquote(rest));
        } else if let Some(rest) = line.strip_prefix("NAME=") {
            if name.is_none() {
                name = Some(unquote(rest));
            }
        } else if let Some(rest) = line.strip_prefix("VERSION=") {
            version = Some(unquote(rest));
        }
    }

    match (name, version) {
        (Some(n), Some(v)) => Some(format!("{} {}", n, v)),
        (Some(n), None) => Some(n),
        _ => None,
    }
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn uptime() -> String {
    match fs::read_to_string("/proc/uptime") {
        Ok(raw) => {
            if let Some(secs_str) = raw.split_whitespace().next() {
                let secs: f64 = secs_str.parse().unwrap_or(0.0);
                format_duration(secs)
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

fn format_duration(secs: f64) -> String {
    let total = secs as u64;
    let days = total / 86400;
    let hours = (total % 86400) / 3600;
    let mins = (total % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if mins > 0 || parts.is_empty() {
        parts.push(format!("{}m", mins));
    }

    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"Arch Linux\""), "Arch Linux");
        assert_eq!(unquote("plain"), "plain");
        assert_eq!(unquote("\"\""), "");
    }

    #[test]
    fn test_format_duration() {
        let f = format_duration(65.0);
        assert!(f.contains("1m"));

        let f = format_duration(3661.0);
        assert!(f.contains("1h"));
        assert!(f.contains("1m"));

        let f = format_duration(90061.0);
        assert!(f.contains("1d"));
        assert!(f.contains("1h"));
        assert!(f.contains("1m"));
    }
}
