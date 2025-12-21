use std::collections::HashMap;
use std::fs;

pub fn get_info() -> Option<HashMap<String, String>> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
    let mut info = HashMap::new();
    for line in meminfo.lines() {
        if let Some((k, v)) = line.split_once(':') {
            info.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }
    Some(info)
}

pub fn kb_to_gb(kb_str: &str) -> f64 {
    let kb = kb_str
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0);
    kb / 1024.0 / 1024.0
}
